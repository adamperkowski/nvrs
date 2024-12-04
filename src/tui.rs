use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use nvrs::*;
use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, ListItem},
    Frame,
};
use tachyonfx::{fx, Duration as FxDuration, Effect, EffectRenderer, Shader};

const KEYBINDS: &str = " [q] Quit  [s] Sync ";

struct AppState {
    is_running: bool,
    is_syncing: bool,
    effect: Effect,

    // nvrs data
    config: (config::Config, std::path::PathBuf),
    verfiles: (verfiles::Verfile, verfiles::Verfile),
    client: reqwest::Client,
    keyfile: Option<keyfile::Keyfile>,
}

impl AppState {
    async fn new() -> Result<Self> {
        let config = config::load(None).await?; // TODO: custom config path
        let verfiles = verfiles::load(&config.0.__config__).await?;
        let keyfile = keyfile::load(&config.0.__config__).await?;

        Ok(Self {
            is_running: true,
            is_syncing: false,
            effect: fx::coalesce(800),

            // nvrs data
            config,
            verfiles,
            client: reqwest::Client::new(),
            keyfile,
        })
    }

    fn draw(&mut self, frame: &mut Frame) {
        let new_names = self.verfiles.1.data.data.keys().collect::<Vec<_>>();

        let list = List::new(
            new_names
                .iter()
                .map(|p| ListItem::new(format!("📦️ {}", p)).style(Style::default().blue())),
        )
        .block(
            Block::bordered()
                .title_top(if self.is_syncing {
                    " Synchronizing... "
                } else {
                    " nvrs "
                })
                .title_bottom(KEYBINDS)
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(list, frame.area());

        if self.effect.running() {
            frame.render_effect(&mut self.effect, frame.area(), FxDuration::from_millis(50))
        }
    }

    async fn sync(&mut self) -> error::Result<()> {
        let config = &self.config.0;

        let tasks: Vec<_> = config
            .packages
            .to_owned()
            .into_iter()
            .map(|p| tokio::spawn(run_source(p, self.client.clone(), self.keyfile.clone())))
            .collect();

        let mut results = futures::future::join_all(tasks).await;

        for package in &config.packages {
            match results.remove(0).unwrap() {
                Ok(release) => {
                    let gitref: String;
                    let tag = if let Some(t) = release.tag.clone() {
                        gitref = format!("refs/tags/{}", t);
                        release.tag.unwrap().replacen(&package.1.prefix, "", 1)
                    } else {
                        gitref = String::new();
                        release.name
                    };

                    if let Some(new_pkg) = self.verfiles.1.data.data.get_mut(package.0) {
                        new_pkg.version = tag.to_string();
                        new_pkg.gitref = gitref;
                        new_pkg.url = release.url;
                    } else {
                        self.verfiles.1.data.data.insert(
                            package.0.clone(),
                            verfiles::VerPackage {
                                version: tag.to_string(),
                                gitref,
                                url: release.url,
                            },
                        );
                    }
                }
                Err(e) => {}
            }
        }

        self.is_syncing = false;

        verfiles::save(&self.verfiles.1, false, config.__config__.clone()).await
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => self.is_syncing = true,
            _ => (),
        }
    }

    fn exit(&mut self) {
        self.is_running = false;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppState::new().await?;
    let mut terminal = ratatui::init();

    while app.is_running {
        terminal.draw(|f| {
            app.draw(f);
        })?;

        if app.effect.running() {
            continue;
        }

        if app.is_syncing {
            app.sync().await?;
        } else {
            app.handle_events()?;
        }
    }

    ratatui::restore();

    Ok(())
}
