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
enum CollectionState {
    Login { input: String },
    Menu { ponto_id: i32, partida_id: i32 },
    InsertingColeta { ponto_id: i32, partida_id: i32, step: u8, aluno_id: String, lixo: String, peso: String },
    Message { ponto_id: Option<i32>, partida_id: Option<i32>, msg: String },
}

thread_local! {
    static STATE: RefCell<CollectionState> = RefCell::new(CollectionState::Login { input: String::new() });
}

pub fn render(_app: &App, frame: &mut Frame) {
    let state = STATE.with(|s| s.borrow().clone());
    let area = frame.area();

    match state {
        CollectionState::Login { input } => {
            let text = vec![
                Line::from("📍 Acesso de Ponto de Coleta 📍".green().bold()),
                Line::from(""),
                Line::from("Para operar as coletas, identifique-se:"),
                Line::from(""),
                Line::from("Digite o ID numérico deste Ponto:".yellow()),
                Line::from(format!("> {}_", input)),
                Line::from(""),
                Line::from("[Enter] Entrar | [ESC] Voltar ao menu principal".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        CollectionState::Menu { ponto_id, partida_id } => {
            let text = vec![
                Line::from(format!("📍 Ponto de Coleta #{} (Partida: {}) 📍", ponto_id, partida_id).green().bold()),
                Line::from(""),
                Line::from("[1] Registrar Nova Coleta de Lixo"),
                Line::from("[ESC] Deslogar"),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        CollectionState::Message { msg, .. } => {
            let text = vec![
                Line::from("Aviso:".bold()),
                Line::from(msg),
                Line::from(""),
                Line::from("[Enter] ou [ESC] para continuar".dark_gray()),
            ];
            let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL)).alignment(Alignment::Center).wrap(Wrap { trim: true });
            frame.render_widget(p, area);
        }
        CollectionState::InsertingColeta { ponto_id: _, partida_id: _, step, aluno_id, lixo, peso } => {
            let prompt = match step {
                0 => "Digite o ID numérico do Aluno:",
                1 => "Categoria de Lixo (Plástico, Alumínio, Vidro):",
                2 => "Peso (em KG, ex: 2.5):",
                _ => "",
            };

            let current_input = match step {
                0 => &aluno_id,
                1 => &lixo,
                2 => &peso,
                _ => "",
            };

            let text = vec![
                Line::from("--- Registrando Coleta ---".green()),
                Line::from(""),
                Line::from(format!("Aluno ID: {}", if step > 0 { &aluno_id } else { "" })),
                Line::from(format!("Categoria Lixo: {}", if step > 1 { &lixo } else { "" })),
                Line::from(format!("Peso (KG): {}", if step > 2 { &peso } else { "" })),
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
        CollectionState::Login { mut input } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = CollectionState::Login { input: String::new() });
                    app.role = UserRole::Guest;
                }
                KeyCode::Backspace => {
                    input.pop();
                    STATE.with(|s| *s.borrow_mut() = CollectionState::Login { input });
                }
                KeyCode::Enter => {
                    authenticate_ponto(&input);
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    STATE.with(|s| *s.borrow_mut() = CollectionState::Login { input });
                }
                _ => {}
            }
        }
        CollectionState::Menu { ponto_id, partida_id } => {
            match key {
                KeyCode::Char('1') => {
                    STATE.with(|s| *s.borrow_mut() = CollectionState::InsertingColeta { ponto_id, partida_id, step: 0, aluno_id: String::new(), lixo: String::new(), peso: String::new() });
                }
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = CollectionState::Login { input: String::new() });
                }
                _ => {}
            }
        }
        CollectionState::Message { ponto_id, partida_id, .. } => {
            match key {
                KeyCode::Enter | KeyCode::Esc => {
                    if let (Some(pid), Some(partid)) = (ponto_id, partida_id) {
                        STATE.with(|s| *s.borrow_mut() = CollectionState::Menu { ponto_id: pid, partida_id: partid });
                    } else {
                        STATE.with(|s| *s.borrow_mut() = CollectionState::Login { input: String::new() });
                    }
                }
                _ => {}
            }
        }
        CollectionState::InsertingColeta { ponto_id, partida_id, mut step, mut aluno_id, mut lixo, mut peso } => {
            match key {
                KeyCode::Esc => {
                    STATE.with(|s| *s.borrow_mut() = CollectionState::Menu { ponto_id, partida_id });
                }
                KeyCode::Enter => {
                    step += 1;
                    if step > 2 {
                        save_coleta(ponto_id, partida_id, &aluno_id, &lixo, &peso);
                    } else {
                        STATE.with(|s| *s.borrow_mut() = CollectionState::InsertingColeta { ponto_id, partida_id, step, aluno_id, lixo, peso });
                    }
                }
                KeyCode::Backspace => {
                    let target = match step {
                        0 => &mut aluno_id,
                        1 => &mut lixo,
                        2 => &mut peso,
                        _ => return,
                    };
                    target.pop();
                    STATE.with(|s| *s.borrow_mut() = CollectionState::InsertingColeta { ponto_id, partida_id, step, aluno_id, lixo, peso });
                }
                KeyCode::Char(c) => {
                    let target = match step {
                        0 => &mut aluno_id,
                        1 => &mut lixo,
                        2 => &mut peso,
                        _ => return,
                    };
                    target.push(c);
                    STATE.with(|s| *s.borrow_mut() = CollectionState::InsertingColeta { ponto_id, partida_id, step, aluno_id, lixo, peso });
                }
                _ => {}
            }
        }
    }
}

fn authenticate_ponto(id_str: &str) {
    let id_val: i32 = match id_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: None, partida_id: None, msg: "ID deve ser numérico!".into() });
            return;
        }
    };

    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: None, partida_id: None, msg: format!("Erro db: {}", e) });
            return;
        }
    };

    let row_opt = client.query_opt("SELECT Partida FROM Ponto_de_Coleta WHERE ID = $1", &[&id_val]);
    match row_opt {
        Ok(Some(row)) => {
            let partida_id: i32 = row.get(0);
            STATE.with(|s| *s.borrow_mut() = CollectionState::Menu { ponto_id: id_val, partida_id });
        }
        Ok(None) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: None, partida_id: None, msg: "Ponto não encontrado!".into() });
        }
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: None, partida_id: None, msg: format!("Erro: {}", e) });
        }
    }
}

fn save_coleta(ponto_id: i32, partida_id: i32, aluno_id_str: &str, lixo: &str, peso_str: &str) {
    let aluno_id: i32 = match aluno_id_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Aluno ID inválido!".into() });
            return;
        }
    };

    let peso_val: f64 = match peso_str.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Peso deve ser um número, ex: 2.5".into() });
            return;
        }
    };

    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: format!("Erro ao conectar: {}", e) });
            return;
        }
    };

    let val_row = match client.query_one(
        "SELECT 
            p.DataHora_Fim < CURRENT_DATE AS is_finished,
            EXISTS(
                SELECT 1 FROM Aluno_Equipe ae 
                INNER JOIN Equipe_Participa_Partida ep ON ae.Nome_Equipe = ep.Nome_Equipe AND ae.Ano_Equipe = ep.Ano_Equipe 
                WHERE ae.Aluno = $1 AND ep.Partida = $2
            ) AS is_participating
        FROM Partida p WHERE p.ID = $2",
        &[&aluno_id, &partida_id]
    ) {
        Ok(r) => r,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: format!("Erro ao validar partida: {}", e) });
            return;
        }
    };

    let is_finished: Option<bool> = val_row.get(0);
    let is_participating: bool = val_row.get(1);

    if is_finished.unwrap_or(false) {
        STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Esta partida já foi encerrada!".into() });
        return;
    }
    
    if !is_participating {
        STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Aluno não participa desta partida!".into() });
        return;
    }

    let row_opt = match client.query_opt("SELECT Pontuacao_KG FROM Lixo WHERE Categoria = $1", &[&lixo]) {
        Ok(r) => r,
        Err(e) => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: format!("Erro de banco: {}", e) });
            return;
        }
    };

    let pontuacao_kg: f64 = match row_opt {
        Some(r) => {
            // Pontuacao_KG is numeric, so cast to float8 in Rust is fine as it's returned as such or we cast in sql... wait.
            // Earlier I did not cast, let me fix it here just in case.
            // Oh, numeric -> f64 might fail.
            0.0 // Placeholder. Let's fix the SQL.
        },
        None => {
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Categoria de lixo não existe!".into() });
            return;
        }
    };
    
    // Fixed query for pontuacao
    let row_opt_fixed = client.query_opt("SELECT Pontuacao_KG::FLOAT8 FROM Lixo WHERE Categoria = $1", &[&lixo]).unwrap();
    let pontuacao_kg: f64 = row_opt_fixed.map(|r| r.get(0)).unwrap_or(0.0);
    
    if pontuacao_kg == 0.0 {
        STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: "Categoria de lixo não existe ou não vale pontos!".into() });
        return;
    }

    let total_pontos = peso_val * pontuacao_kg;

    let res = client.execute(
        "INSERT INTO Coleta (Aluno, Ponto_de_Coleta, Data_Hora, Lixo, Peso, Pontuacao) VALUES ($1, $2, CURRENT_TIMESTAMP, $3, $4::FLOAT8, $5::FLOAT8)",
        &[&aluno_id, &ponto_id, &lixo, &peso_val, &total_pontos]
    );

    match res {
        Ok(_) => {
            let _ = client.execute(
                "UPDATE Equipe_Participa_Partida 
                 SET Pontuacao = Pontuacao + $1::FLOAT8 
                 WHERE Partida = $2 
                 AND Nome_Equipe = (SELECT Nome_Equipe FROM Aluno_Equipe WHERE Aluno = $3) 
                 AND Ano_Equipe = (SELECT Ano_Equipe FROM Aluno_Equipe WHERE Aluno = $3)",
                &[&total_pontos, &partida_id, &aluno_id]
            );
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: format!("Coleta salva! Pontos gerados: {}", total_pontos) })
        },
        Err(e) => {
            let error_msg = e.as_db_error().map(|db_e| db_e.message()).unwrap_or(&e.to_string()).to_string();
            STATE.with(|s| *s.borrow_mut() = CollectionState::Message { ponto_id: Some(ponto_id), partida_id: Some(partida_id), msg: format!("Erro ao salvar: {}", error_msg) })
        },
    }
}
