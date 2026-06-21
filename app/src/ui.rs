use ratatui::Frame;
use crate::app::{App, UserRole};

#[allow(unused_imports)]
use crate::screens::{admin, guest, school, player, analyst, collection_point};

pub fn render(app: &App, frame: &mut Frame) {
    match app.role {
        UserRole::Guest => guest::render(app, frame),
        UserRole::Admin => admin::render(app, frame),
        UserRole::School => school::render(app, frame),
        UserRole::Player => player::render(app, frame),
        UserRole::Analyst => analyst::render(app, frame),
        UserRole::CollectionPoint => collection_point::render(app, frame),
    }
}