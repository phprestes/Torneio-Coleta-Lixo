use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};

use crossterm::event::KeyCode;
use crate::app::{App, UserRole};
use crate::db;

use std::cell::RefCell;

thread_local! {
    static QUERY_RESULT: RefCell<String> = RefCell::new(String::new());
    static SCROLL_OFFSET: RefCell<(u16, u16)> = RefCell::new((0, 0));
}

pub fn render(_app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let menu_text = vec![
        Line::from("📊 Painel do Analista de Dados 📊".cyan().bold()),
        Line::from(""),
        Line::from("Selecione uma consulta para executar:"),
        Line::from("[1] Regiões que coletaram a maior quantidade de lixo por categoria"),
        Line::from("[2] Média de escolas participantes por tipo de campeonato (fase)"),
        Line::from("[3] Ranking das escolas por número de alunos MVP"),
        Line::from("[4] Centros de reciclagem que mais receberam lixo por partida"),
        Line::from("[5] Alunos 'carona' (equipes avançaram, mas não coletaram nada)"),
        Line::from(""),
        Line::from("[ESC] Voltar ao menu principal".dark_gray()),
    ];

    let menu = Paragraph::new(menu_text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Menu de Consultas "))
        .alignment(Alignment::Left);

    frame.render_widget(menu, chunks[0]);

    let res = QUERY_RESULT.with(|q| q.borrow().clone());
    let result_text = if res.is_empty() {
        "Aguardando execução da consulta...".to_string()
    } else {
        res
    };

    let offset = SCROLL_OFFSET.with(|o| *o.borrow());
    let result_p = Paragraph::new(result_text)
        .block(Block::default().borders(Borders::ALL).title(" Resultado (Setas para Rolar) "))
        .alignment(Alignment::Left)
        .scroll((offset.1, offset.0));

    frame.render_widget(result_p, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('1') => { SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0)); execute_query(1); },
        KeyCode::Char('2') => { SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0)); execute_query(2); },
        KeyCode::Char('3') => { SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0)); execute_query(3); },
        KeyCode::Char('4') => { SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0)); execute_query(4); },
        KeyCode::Char('5') => { SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0)); execute_query(5); },
        KeyCode::Up => {
            SCROLL_OFFSET.with(|o| {
                let mut offset = o.borrow_mut();
                if offset.1 > 0 { offset.1 -= 1; }
            });
        },
        KeyCode::Down => {
            SCROLL_OFFSET.with(|o| {
                o.borrow_mut().1 += 1;
            });
        },
        KeyCode::Left => {
            SCROLL_OFFSET.with(|o| {
                let mut offset = o.borrow_mut();
                if offset.0 > 0 { offset.0 -= 1; }
            });
        },
        KeyCode::Right => {
            SCROLL_OFFSET.with(|o| {
                o.borrow_mut().0 += 1;
            });
        },
        KeyCode::Esc => {
            QUERY_RESULT.with(|q| *q.borrow_mut() = String::new());
            SCROLL_OFFSET.with(|o| *o.borrow_mut() = (0, 0));
            app.role = UserRole::Guest;
        }
        _ => {}
    }
}

fn execute_query(query_id: u8) {
    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            set_result(&format!("Erro ao conectar ao banco: {}", e));
            return;
        }
    };

    let sql = match query_id {
        1 => "
            WITH Totais_Regiao AS (
                SELECT c.Lixo AS Categoria, p.Regiao AS Regiao, SUM(c.Peso)::FLOAT8 AS Total_Peso FROM Coleta c
                INNER JOIN Ponto_de_Coleta pc ON c.Ponto_de_Coleta = pc.ID
                INNER JOIN Partida p ON pc.Partida = p.ID
                GROUP BY c.Lixo, p.Regiao
            ),
            Maximos_Categoria AS (
                SELECT Categoria, MAX(Total_Peso)::FLOAT8 AS Maximo_Peso FROM Totais_Regiao
                GROUP BY Categoria
            )
            SELECT tr.Categoria, tr.Regiao, tr.Total_Peso::FLOAT8 FROM Totais_Regiao tr
            INNER JOIN Maximos_Categoria mc ON tr.Categoria = mc.Categoria AND tr.Total_Peso = mc.Maximo_Peso;
        ",
        2 => "
            SELECT Fase AS Tipo_Campeonato, ROUND(AVG(Qtd_Escolas)::NUMERIC, 2)::FLOAT8 AS Media_Escolas 
            FROM (
                SELECT p.Torneio AS Ano, p.Fase AS Fase, COUNT(DISTINCT a.Escola) AS Qtd_Escolas 
                FROM Partida p
                INNER JOIN Equipe_Participa_Partida ep ON p.ID = ep.Partida
                INNER JOIN Aluno a ON ep.Nome_Equipe = a.Nome_Equipe AND ep.Ano_Equipe = a.Ano_Equipe
                GROUP BY p.Torneio, p.Fase
            ) AS Base_Anual
            GROUP BY Fase;
        ",
        3 => "
            SELECT
                e.ID AS Escola_ID, e.Nome AS Nome_Escola, COUNT(p.AlunoMVP) AS Total_MVP FROM Escola e
                LEFT JOIN Aluno a ON e.ID = a.Escola
                LEFT JOIN Partida p ON a.ID = p.AlunoMVP
            GROUP BY e.ID, e.Nome
            ORDER BY Total_MVP DESC;
        ",
        4 => "
            WITH Totais_Por_Centro AS (
                SELECT pc.Partida AS Partida_ID, cr.Nome AS Nome_Centro, SUM(c.Peso)::FLOAT8 AS Total_Recebido FROM Coleta c
                INNER JOIN Ponto_de_Coleta pc ON c.Ponto_de_Coleta = pc.ID
                INNER JOIN Logistica_Transporte lt ON pc.ID = lt.Ponto_de_Coleta
                INNER JOIN Centro_de_Reciclagem cr ON lt.Centro_de_Reciclagem = cr.ID
                GROUP BY pc.Partida, cr.ID, cr.Nome
            )
            SELECT T.Partida_ID, T.Nome_Centro, T.Total_Recebido::FLOAT8 FROM Totais_Por_Centro T
            INNER JOIN (
                SELECT Partida_ID, MAX(Total_Recebido)::FLOAT8 AS Max_Peso FROM Totais_Por_Centro
                GROUP BY Partida_ID
            ) Maximos ON T.Partida_ID = Maximos.Partida_ID AND T.Total_Recebido = Maximos.Max_Peso;
        ",
        5 => "
            SELECT * FROM (
                SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, a.Nome_Equipe FROM Aluno a
                INNER JOIN Escola e ON a.Escola = e.ID
                INNER JOIN Equipe_Participa_Partida ep ON a.Nome_Equipe = ep.Nome_Equipe AND a.Ano_Equipe = ep.Ano_Equipe
                INNER JOIN Partida p ON ep.Partida = p.ID
                WHERE UPPER(p.Fase) IN ('NACIONAL', 'CONTINENTAL', 'INTERNACIONAL')
                EXCEPT
                SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, a.Nome_Equipe FROM Aluno a
                INNER JOIN Escola e ON a.Escola = e.ID
                INNER JOIN Coleta c ON a.ID = c.Aluno
            ) AS Caronas;
        ",
        _ => return,
    };

    match client.query(sql, &[]) {
        Ok(rows) => {
            if rows.is_empty() {
                set_result("A consulta não retornou resultados.");
                return;
            }

            let cols = rows[0].columns();
            let mut output = String::new();

            // Header
            let header: Vec<String> = cols.iter().map(|c| c.name().to_string()).collect();
            output.push_str(&format!("| {} |\n", header.join(" | ")));
            output.push_str(&format!("|{}|\n", vec!["---"; cols.len()].join("|")));

            // Rows
            for row in &rows {
                let mut row_data = Vec::new();
                for (i, col) in cols.iter().enumerate() {
                    let type_name = col.type_().name();
                    let val = match type_name {
                        "int4" => {
                            let v: Option<i32> = row.get(i);
                            v.map(|n| n.to_string()).unwrap_or_else(|| "NULL".to_string())
                        },
                        "int8" => {
                            let v: Option<i64> = row.get(i);
                            v.map(|n| n.to_string()).unwrap_or_else(|| "NULL".to_string())
                        },
                        "varchar" | "text" | "bpchar" => {
                            let v: Option<String> = row.get(i);
                            v.unwrap_or_else(|| "NULL".to_string())
                        },
                        "float8" => {
                            let v: Option<f64> = row.get(i);
                            v.map(|n| n.to_string()).unwrap_or_else(|| "NULL".to_string())
                        },
                        "numeric" => {
                            "<numeric - precisa cast no SQL>".to_string()
                        },
                        _ => "<unsupported>".to_string(),
                    };
                    row_data.push(val);
                }
                output.push_str(&format!("| {} |\n", row_data.join(" | ")));
            }

            set_result(&output);
        }
        Err(e) => {
            set_result(&format!("Erro ao executar consulta: {}", e));
        }
    }
}

fn set_result(res: &str) {
    QUERY_RESULT.with(|q| *q.borrow_mut() = res.to_string());
}
