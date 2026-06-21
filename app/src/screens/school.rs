use ratatui::{
    layout::{Alignment},
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
enum SchoolState {
    Login { input: String },
    Menu { school_id: i32 },
    InsertingTeam { school_id: i32, step: u8, name: String, year: String, tutor_id: String },
    Message { school_id: Option<i32>, msg: String },
}

thread_local! {
    static STATE: RefCell<SchoolState> = RefCell::new(SchoolState::Login { input: String::new() });
}

pub fn render(_app: &App, frame: &mut Frame) {
    let state = STATE.with(|s| s.borrow().clone());
    let area = frame.area();

    match state {
        SchoolState::Login { input } => {
            let text = vec![
                Line::from("🏫 Acesso de Escola 🏫".cyan().bold()),
                Line::from(""),
                Line::from("Para acessar o painel, identifique-se:"),
                Line::from(""),
                Line::from("Digite o ID numérico da sua Escola:".yellow()),
                Line::from(format!("> {}_", input)),
                Line::from(""),
                Line::from("[Enter] Entrar | [ESC] Voltar ao menu principal".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        SchoolState::Menu { school_id } => {
            let text = vec![
                Line::from(format!("🏫 Painel da Escola (ID: {}) 🏫", school_id).cyan().bold()),
                Line::from(""),
                Line::from("[1] Inserir Nova Equipe"),
                Line::from("[ESC] Deslogar"),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        SchoolState::Message { msg, .. } => {
            let text = vec![
                Line::from("Aviso:".bold()),
                Line::from(msg),
                Line::from(""),
                Line::from("[Enter] ou [ESC] para continuar".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center).wrap(Wrap { trim: true });
            frame.render_widget(p, area);
        }
        SchoolState::InsertingTeam { school_id: _, step, name, year, tutor_id } => {
            let prompt = match step {
                0 => "Digite o Nome da Equipe:",
                1 => "Digite o Ano do Torneio (ex: 2026):",
                2 => "Digite o ID numérico do Tutor responsável (que pertence a sua escola):",
                _ => "",
            };

            let current_input = match step {
                0 => &name,
                1 => &year,
                2 => &tutor_id,
                _ => "",
            };

            let text = vec![
                Line::from("--- Cadastrando Nova Equipe ---".yellow()),
                Line::from(""),
                Line::from(format!("Nome: {}", if step > 0 { &name } else { "" })),
                Line::from(format!("Ano: {}", if step > 1 { &year } else { "" })),
                Line::from(format!("Tutor ID: {}", if step > 2 { &tutor_id } else { "" })),
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

pub fn handle_key(app: &mut App, key: KeyCode) {
    let state = STATE.with(|s| s.borrow().clone());
    match state {
        SchoolState::Login { mut input } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = SchoolState::Login { input: String::new() });
                    app.role = UserRole::Guest;
                }
                KeyCode::Backspace => {
                    input.pop();
                    STATE.with(|s| *s.borrow_mut() = SchoolState::Login { input });
                }
                KeyCode::Enter => {
                    authenticate_school(&input);
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    STATE.with(|s| *s.borrow_mut() = SchoolState::Login { input });
                }
                _ => {}
            }
        }
        SchoolState::Menu { school_id } => {
            match key {
                KeyCode::Char('1') => {
                    STATE.with(|s| *s.borrow_mut() = SchoolState::InsertingTeam { school_id, step: 0, name: String::new(), year: String::new(), tutor_id: String::new() });
                }
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = SchoolState::Login { input: String::new() });
                }
                _ => {}
            }
        }
        SchoolState::Message { school_id, .. } => {
            match key {
                KeyCode::Enter | KeyCode::Esc => {
                    if let Some(id) = school_id {
                        STATE.with(|s| *s.borrow_mut() = SchoolState::Menu { school_id: id });
                    } else {
                        STATE.with(|s| *s.borrow_mut() = SchoolState::Login { input: String::new() });
                    }
                }
                _ => {}
            }
        }
        SchoolState::InsertingTeam { school_id, mut step, mut name, mut year, mut tutor_id } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = SchoolState::Menu { school_id });
                }
                KeyCode::Enter => {
                    step += 1;
                    if step > 2 {
                        save_team(school_id, &name, &year, &tutor_id);
                    } else {
                        STATE.with(|s| *s.borrow_mut() = SchoolState::InsertingTeam { school_id, step, name, year, tutor_id });
                    }
                }
                KeyCode::Backspace => {
                    let target = match step {
                        0 => &mut name,
                        1 => &mut year,
                        2 => &mut tutor_id,
                        _ => return,
                    };
                    target.pop();
                    STATE.with(|s| *s.borrow_mut() = SchoolState::InsertingTeam { school_id, step, name, year, tutor_id });
                }
                KeyCode::Char(c) => {
                    let target = match step {
                        0 => &mut name,
                        1 => &mut year,
                        2 => &mut tutor_id,
                        _ => return,
                    };
                    target.push(c);
                    STATE.with(|s| *s.borrow_mut() = SchoolState::InsertingTeam { school_id, step, name, year, tutor_id });
                }
                _ => {}
            }
        }
    }
}

fn authenticate_school(id_str: &str) {
    let id_val: i32 = match id_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: None, msg: "ID deve ser numérico!".into() });
            return;
        }
    };

    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: None, msg: format!("Erro db: {}", e) });
            return;
        }
    };

    let row_opt = client.query_opt("SELECT Nome FROM Escola WHERE ID = $1", &[&id_val]);
    match row_opt {
        Ok(Some(_row)) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Menu { school_id: id_val });
        }
        Ok(None) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: None, msg: "Escola não encontrada!".into() });
        }
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: None, msg: format!("Erro: {}", e) });
        }
    }
}

fn save_team(school_id: i32, name: &str, year_str: &str, tutor_id_str: &str) {
    let year_val: i32 = match year_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: "Ano deve ser um número inteiro!".into() });
            return;
        }
    };
    
    let tutor_val: i32 = match tutor_id_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: "Tutor ID deve ser um número inteiro!".into() });
            return;
        }
    };

    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: format!("Erro ao conectar: {}", e) });
            return;
        }
    };

    // Idealmente deveríamos verificar se o tutor pertence à escola (escola_id = tutor.Escola)
    let check_tutor = client.query_opt("SELECT ID FROM Tutor WHERE ID = $1 AND Escola = $2", &[&tutor_val, &school_id]);
    match check_tutor {
        Ok(Some(_)) => {}, // Tutor ok
        Ok(None) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: "O Tutor não pertence a esta escola!".into() });
            return;
        }
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: format!("Erro ao validar tutor: {}", e) });
            return;
        }
    }

    let res = client.execute(
        "INSERT INTO Equipe (Nome, Ano, Tutor) VALUES ($1, $2, $3)",
        &[&name, &year_val, &tutor_val]
    );

    match res {
        Ok(_) => STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: "Equipe cadastrada com sucesso!".into() }),
        Err(e) => {
            let error_msg = e.as_db_error().map(|db_e| db_e.message()).unwrap_or(&e.to_string()).to_string();
            STATE.with(|s| *s.borrow_mut() = SchoolState::Message { school_id: Some(school_id), msg: format!("Erro ao cadastrar equipe: {}", error_msg) })
        },
    }
}
