use postgres::{Client, NoTls};
use std::env;
use std::fs;

pub fn get_client() -> Result<Client, postgres::Error> {
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

    let db_url = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    
    Client::connect(&db_url, NoTls)
}

pub fn initialize_db() -> Result<(), Box<dyn std::error::Error>> {
    // Carrega o .env (pode falhar silenciosamente caso o var de ambiente já esteja exportado via Makefile)
    dotenvy::from_filename("../.env").ok();

    let mut client = get_client()?;

    let always_recreate = env::var("ALWAYS_RECREATE_DB")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true";

    // Verifica se a tabela Torneio existe
    let row = client.query_opt(
        "SELECT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = 'torneio');",
        &[],
    )?;

    let mut tables_exist = false;
    if let Some(r) = row {
        tables_exist = r.get(0);
    }

    if !tables_exist || always_recreate {
        println!("Criando/Recriando banco de dados...");
        // Drop de tudo
        client.batch_execute("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")?;
        
        // Lê os arquivos sql (assumindo a chamada a partir de /app, as pastas sql estarão em ../sql)
        let esquema_sql = fs::read_to_string("../sql/esquema.sql")?;
        let dados_sql = fs::read_to_string("../sql/dados.sql")?;

        // Roda o esquema e popula
        client.batch_execute(&esquema_sql)?;
        client.batch_execute(&dados_sql)?;
        println!("Banco de dados inicializado com sucesso.");
    } else {
        println!("Banco de dados já existente. Pulando inicialização.");
    }

    Ok(())
}
