use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, // Removido o Rect
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use crossterm::event::KeyCode;

use crate::app::App;

// ==========================================
// 1. RENDERIZAÇÃO (A Visão)
// ==========================================
pub fn render(_app: &App, frame: &mut Frame) {
    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(main_layout[1]);

    // --- CABEÇALHO ---
    let header = Paragraph::new(" PAINEL DO PARTICIPANTE: Partida Atual - Fase Regional ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    frame.render_widget(header, main_layout[0]);

    // --- COLUNA 1: PONTUAÇÃO (Tabela) ---
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let selected_style = Style::default().fg(Color::Green).add_modifier(Modifier::REVERSED);

    // Note que nas tabelas sem cor na linha inteira, também usamos Cell::from() 
    let ranking_rows = vec![
        Row::new(vec![Cell::from("1º"), Cell::from("Equipe Eco (Sua)"), Cell::from("1.540")]).style(selected_style),
        Row::new(vec![Cell::from("2º"), Cell::from("Recicladores do 3ºB"), Cell::from("1.200")]),
        Row::new(vec![Cell::from("3º"), Cell::from("Tropa do Lixo Zero"), Cell::from("950")]),
        Row::new(vec![Cell::from("4º"), Cell::from("Tutor: Prof. Carlos"), Cell::from(Line::from("400").style(Style::default().fg(Color::DarkGray)))]),
    ];

    let ranking_table = Table::new(
        ranking_rows,
        [Constraint::Length(4), Constraint::Min(20), Constraint::Length(10)],
    )
    .header(Row::new(vec!["Pos", "Participante/Equipe", "Pontos"]).style(header_style).bottom_margin(1))
    .block(Block::default().title(" 🏆 Ranking da Partida ").borders(Borders::ALL));
    
    frame.render_widget(ranking_table, body_layout[0]);

    // --- COLUNA 2: PONTOS DE COLETA (Tabela) ---
    // AQUI ESTÁ A CORREÇÃO PRINCIPAL: Envolvendo tudo em Cell::from()
    let collection_rows = vec![
        Row::new(vec![Cell::from("Pátio Central"), Cell::from("Plástico, Metal"), Cell::from("Ativo".green())]),
        Row::new(vec![Cell::from("Cantina"), Cell::from("Orgânico"), Cell::from("Lotado".red())]),
        Row::new(vec![Cell::from("Laboratório"), Cell::from("Eletrônicos, Pilhas"), Cell::from("Ativo".green())]),
        Row::new(vec![Cell::from("Quadra"), Cell::from("Papel, Papelão"), Cell::from("Fechado".dark_gray())]),
    ];

    let collection_table = Table::new(
        collection_rows,
        [Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)],
    )
    .header(Row::new(vec!["Local", "Materiais Aceitos", "Status"]).style(header_style).bottom_margin(1))
    .block(Block::default().title(" 📍 Pontos de Coleta ").borders(Borders::ALL));

    frame.render_widget(collection_table, body_layout[1]);

    // --- RODAPÉ ---
    let footer_text = Line::from(vec![
        " Atalhos: ".into(),
        "[R] ".yellow().bold(), "Registrar Coleta  ".into(),
        "[M] ".yellow().bold(), "Ver Mapa  ".into(),
        "[ESC] ".red().bold(), "Sair".into(),
    ]);

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, main_layout[2]);
}

// ==========================================
// 2. EVENTOS (O Controlador)
// ==========================================
// AQUI ESTÁ A CORREÇÃO DO WARNING: _app
pub fn handle_key(_app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Futuramente: app.current_modal = Modal::RegistrarColeta;
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            // Futuramente: muda para a aba do mapa
        }
        _ => {}
    }
}