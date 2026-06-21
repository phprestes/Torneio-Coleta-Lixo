# Torneio de Coleta de Lixo - FICLE

Projeto desenvolvido para a disciplina **SCC0240 – Bases de Dados** no Instituto de Ciências Matemáticas e de Computação (ICMC-USP), ministrada pela Profa. Elaine Parros Machado de Sousa. O monitor PAE responsável foi Anderson Henrique Giacomini.

## Participantes

- Pedro Henrique de Sousa Prestes – 15507819
- Laura Pazini Medeiros – 15468452
- Fernando Valentim Torres – 15452340
- Pedro Henrique Perez Dias – 15484075
- Frederico Scheffel Oliveira – 15452718

## Sobre o Projeto

O **Torneio de Coleta de Lixo** é uma plataforma que promove o engajamento estudantil e a conscientização socioambiental por meio da gamificação da reciclagem. A competição visa unir as premissas de esportes de coleta, como o "SpoGomi", com infraestrutura de progressão clássica de campeonatos de programação (regional, nacional, continental, mundial).

Este projeto abrange a modelagem conceitual e lógica do banco de dados relacional e a construção de uma Interface Gráfica de Linha de Comando (TUI) em **Rust**, conectada a um banco **PostgreSQL**. A proposta atende aos Objetivos de Desenvolvimento Sustentável (ODS) da ONU: 4 (Educação de Qualidade), 11 (Cidades Sustentáveis) e 12 (Consumo Responsável).

## Tecnologias Utilizadas
- **Rust**: Linguagem base para o protótipo da aplicação CLI (utilizando `ratatui` e `crossterm`).
- **PostgreSQL**: SGBD escolhido para a aplicação.
- **Docker e Docker Compose**: Infraestrutura local da base de dados e ferramentas administrativas (PgAdmin).

## Como Rodar o Projeto

1. **Configuração de Ambiente**:
   Na raiz do projeto, existe um arquivo `model.env`. Caso ainda não possua o arquivo `.env`, crie-o na raiz copiando as variáveis do modelo:
   ```bash
   cp model.env .env
   ```
   > **Aviso**: Por padrão, a variável `ALWAYS_RECREATE_DB=true` no arquivo `.env` fará com que o banco limpe todos os dados e rode os scripts de criação das tabelas toda vez que a aplicação abrir. Para manter dados salvos entre usos, modifique este valor para `false`.

2. **Iniciando a Base de Dados**:
   Utilizando o utilitário `make`, você pode subir a base de dados rapidamente:
   ```bash
   make db-up
   ```

3. **Iniciando a Aplicação**:
   Para iniciar a UI interativa, rode o seguinte comando:
   ```bash
   make run
   ```
   *A compilação do Rust e a criação do schema no PostgreSQL acontecerão automaticamente.*

4. **Encerrando a Base de Dados**:
   Ao finalizar, você pode parar os containers e apagar os volumes de dados com o comando:
   ```bash
   make clean
   ```

## Acesso e Funcionalidades
Pela interface principal, é possível acessar o painel do "Analista de Dados / Estatístico", o qual conta com 5 consultas SQL elaboradas especificamente para avaliar métricas do torneio, como alunos que não pontuam, centros de reciclagem que mais recebem doações e quantidade média de escolas participantes por fase de competição.
