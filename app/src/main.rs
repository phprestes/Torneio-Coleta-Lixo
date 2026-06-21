mod app;
mod events;
mod ui;
mod screens;
pub mod db;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    if let Err(e) = db::initialize_db() {
        eprintln!("Erro ao inicializar o banco de dados: {}", e);
        return Ok(());
    }

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