use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, BorderType, Wrap},
    Frame,
};
use crossterm::event::KeyCode;
use crate::app::{App, UserRole};
use crate::db;
use std::cell::RefCell;

#[derive(Clone)]
enum AdminState {
    Menu,
    InsertingSchool { step: u8, doc_type: String, doc_number: String, country: String, name: String },
    Message(String),
}

thread_local! {
    static STATE: RefCell<AdminState> = RefCell::new(AdminState::Menu);
}

/// Renderiza a interface interativa do painel do Administrador.
pub fn render(_app: &App, frame: &mut Frame) {
    let state = STATE.with(|s| s.borrow().clone());
    let area = frame.area();

    match state {
        AdminState::Menu => {
            let text = vec![
                Line::from("👑 Painel do Administrador 👑".yellow().bold()),
                Line::from(""),
                Line::from("[1] Inserir Nova Escola"),
                Line::from("[ESC] Voltar ao menu principal"),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        AdminState::Message(msg) => {
            let text = vec![
                Line::from("Aviso:".bold()),
                Line::from(msg),
                Line::from(""),
                Line::from("[Enter] ou [ESC] para continuar".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center).wrap(Wrap { trim: true });
            frame.render_widget(p, area);
        }
        AdminState::InsertingSchool { step, doc_type, doc_number, country, name } => {
            let prompt = match step {
                0 => "Digite o Tipo de Documento (ex: CNPJ, SIREN):",
                1 => "Digite o Número do Documento:",
                2 => "Digite a Sigla do País (3 letras, ex: BRA):",
                3 => "Digite o Nome da Escola:",
                _ => "",
            };

            let current_input = match step {
                0 => &doc_type,
                1 => &doc_number,
                2 => &country,
                3 => &name,
                _ => "",
            };

            let text = vec![
                Line::from("--- Cadastrando Nova Escola ---".cyan()),
                Line::from(""),
                Line::from(format!("Tipo Doc: {}", if step > 0 { &doc_type } else { "" })),
                Line::from(format!("Num Doc: {}", if step > 1 { &doc_number } else { "" })),
                Line::from(format!("País: {}", if step > 2 { &country } else { "" })),
                Line::from(format!("Nome: {}", if step > 3 { &name } else { "" })),
                Line::from(""),
                Line::from(prompt.yellow()),
                Line::from(format!("> {}_", current_input)),
                Line::from(""),
                Line::from("[Enter] Próximo | [ESC] Cancelar".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Left);
            frame.render_widget(p, area);
        }
    }
}

/// Processa os eventos de teclado e gerencia a máquina de estados do formulário de administração.
pub fn handle_key(app: &mut App, key: KeyCode) {
    let state = STATE.with(|s| s.borrow().clone());
    match state {
        AdminState::Menu => {
            match key {
                KeyCode::Char('1') => {
                    STATE.with(|s| *s.borrow_mut() = AdminState::InsertingSchool { step: 0, doc_type: String::new(), doc_number: String::new(), country: String::new(), name: String::new() });
                }
                KeyCode::Esc => app.role = UserRole::Guest,
                _ => {}
            }
        }
        AdminState::Message(_) => {
            match key {
                KeyCode::Enter | KeyCode::Esc => STATE.with(|s| *s.borrow_mut() = AdminState::Menu),
                _ => {}
            }
        }
        AdminState::InsertingSchool { mut step, mut doc_type, mut doc_number, mut country, mut name } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = AdminState::Menu);
                }
                KeyCode::Enter => {
                    step += 1;
                    if step > 3 {
                        // Salvar no banco
                        save_school(&doc_type, &doc_number, &country, &name);
                    } else {
                        STATE.with(|s| *s.borrow_mut() = AdminState::InsertingSchool { step, doc_type, doc_number, country, name });
                    }
                }
                KeyCode::Backspace => {
                    let target = match step {
                        0 => &mut doc_type,
                        1 => &mut doc_number,
                        2 => &mut country,
                        3 => &mut name,
                        _ => return,
                    };
                    target.pop();
                    STATE.with(|s| *s.borrow_mut() = AdminState::InsertingSchool { step, doc_type, doc_number, country, name });
                }
                KeyCode::Char(c) => {
                    let target = match step {
                        0 => &mut doc_type,
                        1 => &mut doc_number,
                        2 => &mut country,
                        3 => &mut name,
                        _ => return,
                    };
                    target.push(c);
                    STATE.with(|s| *s.borrow_mut() = AdminState::InsertingSchool { step, doc_type, doc_number, country, name });
                }
                _ => {}
            }
        }
    }
}

/// Salva a nova escola no banco de dados, utilizando transações seguras contra injeção SQL.
fn save_school(doc_type: &str, doc_number: &str, country: &str, name: &str) {
    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = AdminState::Message(format!("Erro ao conectar: {}", e)));
            return;
        }
    };

    // Prevenção a SQL Injection nativa usando prepared statements $1, $2, etc.
    let res = client.execute(
        "INSERT INTO Escola (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES ($1, $2, $3::CHAR(3), $4)",
        &[&doc_type, &doc_number, &country, &name]
    );

    match res {
        Ok(_) => STATE.with(|s| *s.borrow_mut() = AdminState::Message("Escola cadastrada com sucesso!".into())),
        Err(e) => {
            let error_msg = e.as_db_error().map(|db_e| db_e.message()).unwrap_or(&e.to_string()).to_string();
            STATE.with(|s| *s.borrow_mut() = AdminState::Message(format!("Erro ao cadastrar escola: {}", error_msg)))
        },
    }
}
