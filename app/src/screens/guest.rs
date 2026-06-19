use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect, Flex},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crossterm::event::KeyCode;

use crate::app::{App, UserRole};

pub fn render(_app: &App, frame: &mut Frame) {
    let area = centered_rect(80, 28, frame.area());

    // Fatiando a nossa "janela" central em duas partes: Menu e Patrocinadores
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(20),
            Constraint::Length(6),
        ])
        .split(area);

    let main_block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Green))
        .title_alignment(Alignment::Center);

    let main_text = vec![
        Line::from("  _______  _____  _       ".green().bold()),
        Line::from(" |__   __|/ ____|| |      ".green().bold()),
        Line::from("    | |  | |     | |      ".green().bold()),
        Line::from("    | |  | |____ | |____  ".green().bold()),
        Line::from("    |_|   \\_____||______| ".green().bold()),
        Line::from(""),
        Line::from("Torneio de Coleta de Lixo - Edição 2026".cyan().bold()),
        Line::from(""),
        Line::from("Selecione seu perfil de acesso:".underlined()),
        Line::from(""),
        Line::from(vec!["[1] ".yellow().bold(), "Entrar como Administrador".into()]),
        Line::from(vec!["[2] ".yellow().bold(), "Entrar como Organizador".into()]),
        Line::from(vec!["[3] ".yellow().bold(), "Entrar como Jogador/Coletor".into()]),
        Line::from(vec!["[Q] ".red().bold(), "Sair do Sistema".into()]),
        Line::from(""),
        Line::from("Uma iniciativa da".italic()),
        Line::from("♻ Federação Internacional da Coleta de Lixo Esportiva (FICLE) ♻".green()),
    ];

    let menu = Paragraph::new(main_text)
        .block(main_block)
        .alignment(Alignment::Center);

    let sponsors_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Patrocinadores Oficiais ")
        .title_alignment(Alignment::Center);

    let sponsors_text = vec![
        Line::from(""),
        // MOCK PATROCINADORES
        Line::from("Prefeitura Municipal  •  ONG Planeta Verde  •  ReciclaMais S.A."),
        Line::from(""),
        Line::from("Apoiando a sustentabilidade e a tecnologia".italic().dark_gray()),
    ];

    let sponsors = Paragraph::new(sponsors_text)
        .block(sponsors_block)
        .alignment(Alignment::Center);

    frame.render_widget(menu, popup_layout[0]);
    frame.render_widget(sponsors, popup_layout[1]);
}

/// Função auxiliar que calcula o centro exato da tela.
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center).areas(r);
    let [area] = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center).areas(area);
    area
}

pub fn handle_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('1') => app.role = UserRole::Admin,
        KeyCode::Char('2') => app.role = UserRole::School,
        KeyCode::Char('3') => app.role = UserRole::Player,
        _ => {}
    }
}