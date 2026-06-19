mod app;
mod events;
mod ui;
mod screens;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| ui::render(app, frame))?;
        events::handle_events(app)?;
    }
    Ok(())
}