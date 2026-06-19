use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, UserRole};
use crate::screens;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            
            // ATALHOS GLOBAIS
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    app.exit = true;
                    return Ok(());
                }
                KeyCode::Esc => {
                    // Volta pro menu inicial
                    app.role = UserRole::Guest; 
                    return Ok(());
                }
                _ => {}
            }

            // ROTEADOR DE TELAS
            match app.role {
                UserRole::Guest => screens::guest::handle_key(app, key.code),
                // UserRole::Admin => screens::admin::handle_key(app, key.code),
                // UserRole::Organizer => screens::organizer::handle_key(app, key.code),
                // UserRole::Player => screens::player::handle_key(app, key.code),
                _ => {} 
            }
        }
    }
    Ok(())
}