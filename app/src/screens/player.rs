use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};
use crossterm::event::KeyCode;
use crate::app::{App, UserRole};
use crate::db;
use std::cell::RefCell;

#[derive(Clone)]
enum PlayerState {
    Login { input: String },
    Dashboard { aluno_id: i32, equipe: String, partida_id: i32 },
    Message(String),
}

thread_local! {
    static STATE: RefCell<PlayerState> = RefCell::new(PlayerState::Login { input: String::new() });
}

pub fn render(_app: &App, frame: &mut Frame) {
    let state = STATE.with(|s| s.borrow().clone());
    let area = frame.area();

    match state {
        PlayerState::Login { input } => {
            let text = vec![
                Line::from("🎮 Acesso de Jogador/Aluno 🎮".cyan().bold()),
                Line::from(""),
                Line::from("Para acessar suas partidas, identifique-se:"),
                Line::from(""),
                Line::from("Digite seu ID de Aluno:".yellow()),
                Line::from(format!("> {}_", input)),
                Line::from(""),
                Line::from("[Enter] Entrar | [ESC] Voltar ao menu principal".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        PlayerState::Message(msg) => {
            let text = vec![
                Line::from("Aviso:".bold()),
                Line::from(msg),
                Line::from(""),
                Line::from("[Enter] ou [ESC] para continuar".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center).wrap(Wrap { trim: true });
            frame.render_widget(p, area);
        }
        PlayerState::Dashboard { aluno_id: _, equipe, partida_id } => {
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

            let header = Paragraph::new(format!(" PAINEL DO JOGADOR | Equipe: {} | Partida: {} ", equipe, partida_id))
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
            frame.render_widget(header, main_layout[0]);

            let mut ranking_rows = Vec::new();
            let mut collection_rows = Vec::new();
            
            if let Ok(mut client) = db::get_client() {
                let sql_rank = "SELECT Nome_Equipe, Pontuacao::FLOAT8 FROM Equipe_Participa_Partida WHERE Partida = $1 ORDER BY Pontuacao DESC";
                if let Ok(rows) = client.query(sql_rank, &[&partida_id]) {
                    for (i, row) in rows.iter().enumerate() {
                        let nome: String = row.get(0);
                        let pontos: f64 = row.get(1);
                        
                        let is_own = nome == equipe;
                        let row_style = if is_own {
                            Style::default().fg(Color::Green).add_modifier(Modifier::REVERSED)
                        } else {
                            Style::default()
                        };

                        ranking_rows.push(Row::new(vec![
                            Cell::from(format!("{}º", i + 1)),
                            Cell::from(if is_own { format!("{} (Sua Equipe)", nome) } else { nome }),
                            Cell::from(format!("{:.2}", pontos)),
                        ]).style(row_style));
                    }
                } else {
                    ranking_rows.push(Row::new(vec![Cell::from("Erro ao carregar ranking")]));
                }

                let sql_pontos = "SELECT ID, Latitude::FLOAT8, Longitude::FLOAT8 FROM Ponto_de_Coleta WHERE Partida = $1";
                if let Ok(rows) = client.query(sql_pontos, &[&partida_id]) {
                    for row in rows.iter() {
                        let id: i32 = row.get(0);
                        let lat: f64 = row.get(1);
                        let lon: f64 = row.get(2);
                        collection_rows.push(Row::new(vec![
                            Cell::from(format!("Ponto #{}", id)),
                            Cell::from(format!("{:.4}, {:.4}", lat, lon)),
                        ]));
                    }
                } else {
                    collection_rows.push(Row::new(vec![Cell::from("Erro ao carregar pontos")]));
                }
            }

            let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

            let ranking_table = Table::new(
                ranking_rows,
                [Constraint::Length(4), Constraint::Min(20), Constraint::Length(10)],
            )
            .header(Row::new(vec!["Pos", "Equipe", "Pontos"]).style(header_style).bottom_margin(1))
            .block(Block::default().title(" 🏆 Ranking da Partida ").borders(Borders::ALL));
            
            frame.render_widget(ranking_table, body_layout[0]);

            let collection_table = Table::new(
                collection_rows,
                [Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)],
            )
            .header(Row::new(vec!["Local", "Coordenadas"]).style(header_style).bottom_margin(1))
            .block(Block::default().title(" 📍 Pontos de Coleta ").borders(Borders::ALL));

            frame.render_widget(collection_table, body_layout[1]);

            let footer_text = Line::from(vec![
                " [ESC] ".red().bold(), "Deslogar e Voltar".into(),
            ]);

            let footer = Paragraph::new(footer_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(footer, main_layout[2]);
        }
    }
}

pub fn handle_key(app: &mut App, key: KeyCode) {
    let state = STATE.with(|s| s.borrow().clone());
    match state {
        PlayerState::Login { mut input } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = PlayerState::Login { input: String::new() });
                    app.role = UserRole::Guest;
                }
                KeyCode::Backspace => {
                    input.pop();
                    STATE.with(|s| *s.borrow_mut() = PlayerState::Login { input });
                }
                KeyCode::Enter => {
                    authenticate_player(&input);
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    STATE.with(|s| *s.borrow_mut() = PlayerState::Login { input });
                }
                _ => {}
            }
        }
        PlayerState::Message(_) => {
            match key {
                KeyCode::Enter | KeyCode::Esc => STATE.with(|s| *s.borrow_mut() = PlayerState::Login { input: String::new() }),
                _ => {}
            }
        }
        PlayerState::Dashboard { .. } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = PlayerState::Login { input: String::new() });
                }
                _ => {}
            }
        }
    }
}

fn authenticate_player(id_str: &str) {
    let id_val: i32 = match id_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = PlayerState::Message("ID deve ser numérico!".into()));
            return;
        }
    };

    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = PlayerState::Message(format!("Erro db: {}", e)));
            return;
        }
    };

    let sql = "
        SELECT ae.Nome_Equipe, ep.Partida 
        FROM Aluno a
        INNER JOIN Aluno_Equipe ae ON ae.Aluno = a.ID
        LEFT JOIN Equipe_Participa_Partida ep ON ae.Nome_Equipe = ep.Nome_Equipe AND ae.Ano_Equipe = ep.Ano_Equipe
        LEFT JOIN Partida p ON p.ID = ep.Partida
        WHERE a.ID = $1 
        ORDER BY p.DataHora_Inicio DESC NULLS LAST
        LIMIT 1
    ";

    let row_opt = client.query_opt(sql, &[&id_val]);
    match row_opt {
        Ok(Some(row)) => {
            let equipe: String = row.get(0);
            let partida_id_opt: Option<i32> = row.get(1);
            if let Some(partida_id) = partida_id_opt {
                STATE.with(|s| *s.borrow_mut() = PlayerState::Dashboard { aluno_id: id_val, equipe, partida_id });
            } else {
                STATE.with(|s| *s.borrow_mut() = PlayerState::Message("Sua equipe ainda não está alocada em nenhuma partida!".into()));
            }
        }
        Ok(None) => {
            STATE.with(|s| *s.borrow_mut() = PlayerState::Message("Aluno não encontrado!".into()));
        }
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = PlayerState::Message(format!("Erro: {}", e)));
        }
    }
}