use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use nvrs::*;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List},
    Frame,
};
use tachyonfx::{fx, Duration as FxDuration, Effect, EffectRenderer, Shader};

const KEYBINDS: &str = " [q] Quit  [s] Sync ";

struct AppState {
    is_running: bool,
    is_syncing: bool,
    is_comparing: bool,
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
            is_comparing: true,
            effect: fx::coalesce(800),

            // nvrs data
            config,
            verfiles,
            client: reqwest::Client::new(),
            keyfile,
        })
    }

    fn draw(&mut self, frame: &mut Frame) {
        if self.is_comparing {
            self.draw_compare(frame);
        }

        if self.effect.running() {
            frame.render_effect(&mut self.effect, frame.area(), FxDuration::from_millis(50))
        }
    }

    fn draw_compare(&mut self, frame: &mut Frame) {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let title = if self.is_syncing {
            " synchronizing... "
        } else {
            " newver "
        };

        let new_data = &self.verfiles.1.data.data;
        let old_data = &self.verfiles.0.data.data;

        let mut new_items: Vec<Line> = Vec::with_capacity(new_data.len());
        let mut old_items: Vec<Line> = Vec::with_capacity(old_data.len());

        for new in new_data.iter() {
            let old = old_data.iter().find(|old| old.0 == new.0);

            let style = if let Some(old) = old {
                if new.1.version != old.1.version {
                    (Style::new().fg(Color::Green), Style::new().fg(Color::Red))
                } else {
                    (Style::default(), Style::default())
                }
            } else {
                (Style::new().fg(Color::Yellow), Style::default())
            };
            let blue = Style::new().fg(Color::Blue);

            let name = format!("{} ", new.0);

            let new_line = Line::from_iter([
                "📦️ ".into(),
                Span::styled(name.clone(), blue),
                Span::styled(new.1.version.clone(), style.0),
            ]);

            let old_line = if let Some(old) = old {
                Line::from_iter([
                    "📦️ ".into(),
                    Span::styled(name, blue),
                    Span::styled(old.1.version.clone(), style.1),
                ])
            } else {
                Line::from("")
            };

            new_items.push(new_line);
            old_items.push(old_line);
        }

        let new_list = List::new(new_items).block(
            Block::bordered()
                .title_top(title)
                .border_type(BorderType::Rounded),
        );
        let old_list = List::new(old_items).block(
            Block::bordered()
                .title_top(" oldver ")
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(new_list, layout[0]);
        frame.render_widget(old_list, layout[1]);
    }

    async fn sync(&mut self) -> error::Result<()> {
        let config = &self.config.0;

        let tasks: Vec<_> = config
            .packages
            .clone()
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
