use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, BorderType, Table, Row, Cell},
    Frame,
};

use crossterm::event::KeyCode;
use crate::app::{App, UserRole};
use crate::db;

use std::cell::RefCell;

thread_local! {
    static QUERY_RESULT: RefCell<Result<(Vec<String>, Vec<Vec<String>>), String>> = RefCell::new(Err(String::new()));
}

/// Renderiza a interface de Analista de Dados, exibindo o menu lateral e os resultados da query em tabela.
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
    
    match res {
        Ok((header, rows_data)) => {
            let num_cols = header.len() as u16;
            let widths: Vec<Constraint> = (0..num_cols).map(|_| Constraint::Percentage(100 / num_cols)).collect();
            
            let header_cells = header.iter().map(|h| Cell::from(h.clone()).style(Style::default().fg(Color::Yellow)));
            let header_row = Row::new(header_cells).bottom_margin(1);
            
            let mut table_rows = Vec::new();
            for r in rows_data {
                let cells = r.iter().map(|c| Cell::from(c.clone()));
                table_rows.push(Row::new(cells));
            }

            let table = Table::new(table_rows, widths)
                .header(header_row)
                .block(Block::default().borders(Borders::ALL).title(" Resultado "));
            frame.render_widget(table, chunks[1]);
        }
        Err(msg) => {
            let result_text = if msg.is_empty() {
                "Aguardando execução da consulta...".to_string()
            } else {
                msg
            };

            let result_p = Paragraph::new(result_text)
                .block(Block::default().borders(Borders::ALL).title(" Resultado "))
                .alignment(Alignment::Left);

            frame.render_widget(result_p, chunks[1]);
        }
    }
}

/// Mapeia os botões de 1 a 5 para engatilhar consultas e o Esc para deslogar do Analista.
pub fn handle_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('1') => execute_query(1),
        KeyCode::Char('2') => execute_query(2),
        KeyCode::Char('3') => execute_query(3),
        KeyCode::Char('4') => execute_query(4),
        KeyCode::Char('5') => execute_query(5),
        KeyCode::Esc => {
            QUERY_RESULT.with(|q| *q.borrow_mut() = Err(String::new()));
            app.role = UserRole::Guest;
        }
        _ => {}
    }
}

/// Roda o statement SQL selecionado no banco de dados e repassa o resultado para estado em tela.
fn execute_query(query_id: u8) {
    let mut client = match db::get_client() {
        Ok(c) => c,
        Err(e) => {
            set_result(Err(format!("Erro ao conectar ao banco: {}", e)));
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
                INNER JOIN Aluno_Equipe ae ON ep.Nome_Equipe = ae.Nome_Equipe AND ep.Ano_Equipe = ae.Ano_Equipe
                INNER JOIN Aluno a ON ae.Aluno = a.ID
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
                SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, ae.Nome_Equipe FROM Aluno a
                INNER JOIN Escola e ON a.Escola = e.ID
                INNER JOIN Aluno_Equipe ae ON ae.Aluno = a.ID
                INNER JOIN Equipe_Participa_Partida ep ON ae.Nome_Equipe = ep.Nome_Equipe AND ae.Ano_Equipe = ep.Ano_Equipe
                INNER JOIN Partida p ON ep.Partida = p.ID
                WHERE UPPER(p.Fase) IN ('NACIONAL', 'CONTINENTAL', 'INTERNACIONAL')
                EXCEPT
                SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, ae.Nome_Equipe FROM Aluno a
                INNER JOIN Escola e ON a.Escola = e.ID
                INNER JOIN Aluno_Equipe ae ON ae.Aluno = a.ID
                INNER JOIN Coleta c ON a.ID = c.Aluno
            ) Alunos_Caronas
            ORDER BY ID_Aluno;
        ",
        _ => "",
    };

    match client.query(sql, &[]) {
        Ok(rows) => {
            if rows.is_empty() {
                set_result(Err("A consulta não retornou resultados.".into()));
                return;
            }

            let cols = rows[0].columns();
            let header: Vec<String> = cols.iter().map(|c| c.name().to_string()).collect();
            
            let mut all_rows = Vec::new();
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
                all_rows.push(row_data);
            }

            set_result(Ok((header, all_rows)));
        }
        Err(e) => {
            set_result(Err(format!("Erro ao executar consulta: {}", e)));
        }
    }
}

/// Popula a thread local com os dados prontos (Células) a fim do painel da TUI conseguir exibi-los.
fn set_result(res: Result<(Vec<String>, Vec<Vec<String>>), String>) {
    QUERY_RESULT.with(|q| *q.borrow_mut() = res);
}
