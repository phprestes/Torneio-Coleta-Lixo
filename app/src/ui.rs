use ratatui::Frame;
use crate::app::{App, UserRole};

#[allow(unused_imports)]
use crate::screens::{admin, guest, school, player};

pub fn render(app: &App, frame: &mut Frame) {
    match app.role {
        UserRole::Guest => guest::render(app, frame),
        UserRole::Admin => todo!(),
        UserRole::School => todo!(),
        UserRole::Player => player::render(app, frame),
    }
}