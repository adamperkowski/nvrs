use anyhow::Result;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{
    layout::Alignment, style::Color, widgets::{Block, BorderType, Borders}, Frame
};
use tachyonfx::{fx, Duration as FxDuration, Effect, EffectRenderer, Shader};

struct AppState {
    is_running: bool,
    effect: Effect,
}

impl AppState {
    async fn new() -> Result<Self> {
        Ok(Self { is_running: true, effect: fx::fade_to_fg(Color::White, FxDuration::from_millis(500)) })
    }

    fn draw(&mut self, frame: &mut Frame) {
        let block = Block::default()
            .title(" nvrs ").title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        frame.render_widget(block, frame.area());

        if self.effect.running() {
            frame.render_effect(&mut self.effect, frame.area(), FxDuration::from_millis(100));
        }
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
            _ => {}
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
            tokio::time::sleep(FxDuration::from_millis(100).into()).await;
            continue;
        }

        app.handle_events()?;
    }

    ratatui::restore();

    Ok(())
}
