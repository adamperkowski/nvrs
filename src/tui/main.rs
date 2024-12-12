use anyhow::Result;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = state::App::new()
        .await
        .map_err(|e| {
            e.pretty();
            std::process::exit(1)
        })
        .unwrap();
    let mut terminal = ratatui::init();

    while app.is_running {
        terminal.draw(|f| app.draw(f))?;

        if app.effect_is_running() {
            continue;
        }

        if app.is_syncing {
            app.sync().await?;
        } else {
            app.handle_events()?;
        }
    }

    app.effect_reverse();

    while app.effect_is_running() {
        terminal.draw(|f| app.draw(f))?;
    }

    ratatui::restore();

    Ok(())
}
