use super::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use nvrs::*;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListState, Paragraph},
    Frame,
};
use tachyonfx::{fx, Duration as FxDuration, Effect, EffectRenderer, Shader};

const KEYBINDS: &str = " [q]uit  [s]ync  [f]ilter updated  [/] search ";
const KEYBINDS_SELECTED: &str =
    " [q]uit  [s]ync  [enter] take  [n]uke  [f]ilter updated  [/] search ";
const KEYBINDS_SEARCH: &str = " [esc] cancel  [enter] search ";

const PACKAGE_ICON: &str = " ";

pub struct App {
    pub is_running: bool,
    pub is_syncing: bool,
    is_comparing: bool,
    filter_updated: bool,
    effect: Effect,
    items: Vec<api::Release>,
    search_input: Vec<char>,
    is_searching: bool,
    completion_preview: Option<String>,
    list_state: ListState,

    // nvrs data
    config: (config::Config, std::path::PathBuf),
    verfiles: (verfiles::Verfile, verfiles::Verfile),
    client: reqwest::Client,
    keyfile: Option<keyfile::Keyfile>,
}

impl App {
    pub async fn new() -> error::Result<Self> {
        let config = config::load(&None).await?; // TODO: custom config path
        let verfiles = verfiles::load(&config.0.__config__).await?;
        let keyfile = keyfile::load(&config.0.__config__).await?;

        Ok(Self {
            is_running: true,
            is_syncing: false,
            is_comparing: true,
            filter_updated: false,
            effect: fx::coalesce(800),
            items: Vec::new(),
            search_input: Vec::new(),
            is_searching: false,
            completion_preview: None,
            list_state: {
                let mut state = ListState::default();
                state.select(Some(0));
                state
            },

            // nvrs data
            config,
            verfiles,
            client: reqwest::Client::new(),
            keyfile,
        })
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        if self.is_comparing {
            self.draw_compare(frame);
        }

        if self.effect.running() {
            frame.render_effect(&mut self.effect, frame.area(), FxDuration::from_millis(50))
        }
    }

    fn draw_compare(&mut self, frame: &mut Frame) {
        let vertical =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(3)]).split(frame.area());
        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(vertical[0]);

        let new_data = &self.verfiles.1.data.data;
        let old_data = &self.verfiles.0.data.data;

        let mut new_items: Vec<Line> = Vec::with_capacity(new_data.len());
        let mut old_items: Vec<Line> = Vec::with_capacity(old_data.len());

        for (index, new) in new_data.iter().enumerate() {
            if !self.search_input.is_empty() {
                let search = self.search_input.iter().collect::<String>();
                if !new.0.contains(&search) {
                    continue;
                }
            }

            let old = old_data.iter().find(|old| old.0 == new.0);

            let (style, display) = if let Some(old) = old {
                if new.1.version != old.1.version {
                    (
                        (Style::new().fg(Color::Green), Style::new().fg(Color::Red)),
                        true,
                    )
                } else {
                    ((Style::default(), Style::default()), false)
                }
            } else {
                ((Style::new().fg(Color::Yellow), Style::default()), true)
            };
            let blue = Style::new().fg(Color::Blue);

            let name = format!("{} ", new.0);

            let selected_style = if Some(index) == self.list_state.selected() {
                Style::new().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            let new_line = Line::from_iter([
                PACKAGE_ICON.into(),
                Span::styled(name.clone(), blue),
                Span::styled(&new.1.version, style.0),
            ])
            .style(selected_style);

            let old_line = if let Some(old) = old {
                Line::from_iter([
                    PACKAGE_ICON.into(),
                    Span::styled(name, blue),
                    Span::styled(&old.1.version, style.1),
                ])
                .style(selected_style)
            } else {
                Line::from_iter([
                    PACKAGE_ICON.into(),
                    Span::styled("NONE", Style::new().fg(Color::Red)),
                ])
                .style(selected_style)
            };

            if self.filter_updated && !display {
                continue;
            } else {
                new_items.push(new_line);
                old_items.push(old_line);
            }
        }

        let new_list = List::new(new_items).block(
            Block::bordered()
                .title_top(" newver ")
                .border_type(BorderType::Rounded),
        );
        let old_list = List::new(old_items).block(
            Block::bordered()
                .title_top(" oldver ")
                .border_type(BorderType::Rounded),
        );

        frame.render_stateful_widget(new_list, horizontal[0], &mut self.list_state);
        frame.render_stateful_widget(old_list, horizontal[1], &mut self.list_state);

        self.draw_searchbar(frame, vertical[1]);
    }

    fn draw_searchbar(&self, frame: &mut Frame, area: Rect) {
        let title = if self.is_syncing {
            " synchronizing... "
        } else if !self.search_input.is_empty() {
            " search "
        } else if self.filter_updated {
            " filtered "
        } else {
            " nvrs "
        };

        let content = self.search_input.iter().collect::<String>();
        let display_text = if !self.is_searching && content.is_empty() {
            Span::styled("search", Style::default().fg(Color::DarkGray))
        } else {
            Span::raw(&content)
        };

        let search_bar = Paragraph::new(display_text).block(
            Block::bordered()
                .title_top(title)
                .title_bottom(if self.is_searching {
                    KEYBINDS_SEARCH
                } else {
                    self.list_state
                        .selected()
                        .map_or(KEYBINDS, |_| KEYBINDS_SELECTED)
                })
                .title_alignment(ratatui::layout::Alignment::Center)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(search_bar, area);
    }

    fn update_completion_preview(&mut self) {}

    pub async fn sync(&mut self) -> error::Result<()> {
        let config = &self.config.0;

        let tasks: Vec<_> = config
            .packages
            .clone()
            .into_iter()
            .map(|p| tokio::spawn(run_source(p, self.client.clone(), self.keyfile.clone())))
            .collect();

        let mut results = futures::future::join_all(tasks).await;

        for package in &config.packages {
            // TODO: error popups
            if let Ok(release) = results.remove(0).unwrap() {
                let gitref: String;
                let tag = if let Some(t) = release.tag {
                    gitref = format!("refs/tags/{}", t);
                    t.replacen(&package.1.prefix, "", 1)
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
                        package.0.to_string(),
                        verfiles::VerPackage {
                            version: tag.to_string(),
                            gitref,
                            url: release.url,
                        },
                    );
                }
            }
        }

        self.list_state.select(Some(0));
        self.is_syncing = false;

        verfiles::save(&self.verfiles.1, false, &config.__config__).await
    }

    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.is_searching {
            match key_event.code {
                KeyCode::Esc => {
                    self.is_searching = false;
                    self.search_input.clear();
                }
                KeyCode::Backspace => {
                    self.search_input.pop();
                }
                KeyCode::Enter => {
                    self.list_state.select(Some(0));
                    self.is_searching = false;
                }
                KeyCode::Char(c) => self.search_input.push(c),
                _ => {}
            }
        } else {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('s') => self.is_syncing = true,
                KeyCode::Char('f') => self.filter_updated = !self.filter_updated,
                KeyCode::Char('/') => self.is_searching = true,
                KeyCode::Down | KeyCode::Char('j') => {
                    if self
                        .list_state
                        .selected()
                        .is_some_and(|s| s + 1 < self.verfiles.1.data.data.len())
                    {
                        self.list_state.select_next()
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
                _ => (),
            }
        }
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn effect_is_running(&self) -> bool {
        self.effect.running()
    }

    pub fn effect_reverse(&mut self) {
        self.effect.reverse();
        self.effect.reset();
    }
}
