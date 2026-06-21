<div align="center">
  <h1>♻️ Torneio de Coleta de Lixo - FICLE ♻️</h1>
  <p><strong>Uma plataforma inovadora para promover engajamento estudantil e conscientização socioambiental através da gamificação!</strong></p>
</div>

<br>

## 📖 Sobre o Projeto
O **Torneio de Coleta de Lixo** une esportes de coleta (como o famoso *SpoGomi*) a uma infraestrutura clássica de campeonatos. As escolas, equipes e alunos competem desde níveis regionais até o mundial para ver quem consegue coletar a maior quantidade e variedade de lixo reciclável!

> 🏆 Promovido pela fictícia **Federação Internacional da Coleta de Lixo Esportiva (FICLE)**, este projeto atende diretamente os Objetivos de Desenvolvimento Sustentável (ODS) da ONU: **4 (Educação de Qualidade), 11 (Cidades Sustentáveis) e 12 (Consumo Responsável)**.

Este repositório contém a **modelagem relacional** (PostgreSQL) e o protótipo de uma **Interface Gráfica de Linha de Comando (TUI)** desenvolvida em **Rust**.

<br>

## 🛠 Tecnologias Utilizadas

- 🦀 **Rust**: Criação da TUI moderna e interativa (via bibliotecas `ratatui` e `crossterm`).
- 🐘 **PostgreSQL**: SGBD relacional responsável por gerenciar as complexidades das partidas, alunos e lixos.
- 🐳 **Docker / Docker Compose**: Infraestrutura local ágil (inclui o banco de dados e o PgAdmin).

<br>

## 🚀 Como Rodar o Projeto

Siga os passos abaixo para ter a interface rodando em poucos segundos na sua máquina:

### 1️⃣ Configurando o Ambiente
Primeiro, garanta que você tenha o arquivo de variáveis de ambiente. Na raiz do projeto, basta copiar o arquivo modelo:
```bash
cp model.env .env
```
> ⚠️ **Aviso:** O `.env` possui a flag `ALWAYS_RECREATE_DB=true`. Ela fará com que o banco apague todos os dados e repopule o ambiente de testes (Mocks) toda vez que você abrir o app. Para manter seus registros salvos permanentemente, altere-a para `false`.

### 2️⃣ Subindo a Infraestrutura
Com o Docker aberto, utilize nosso `Makefile` para inicializar a rede de contêineres e a base de dados:
```bash
make db-up
```

### 3️⃣ Abrindo a Aplicação
Com o banco pronto, inicialize a interface interativa em Rust! (O schema SQL e as dependências serão processadas automaticamente):
```bash
make run
```

### 🧹 Limpando tudo
Ao terminar de explorar, você pode encerrar e destruir os volumes temporários com segurança:
```bash
make clean
```

<br>

## 🎮 Funcionalidades do Sistema

Nossa TUI (`Terminal User Interface`) simula a integração de vários atores dentro do ecossistema do campeonato:
- 👑 **Administrador:** Cadastra e gerencia entidades raiz, como Escolas.
- 🏫 **Organizador (Escolas):** Gerencia equipes de alunos que vão participar.
- 🏃 **Jogador (Equipes):** Acompanha a classificação da sua equipe e os dados geográficos da sua partida atual.
- 🏭 **Ponto de Coleta:** Responsável por registrar a pesagem do lixo por aluno, pontuando a equipe dinamicamente em tempo real.
- 📊 **Analista de Dados:** Um painel gerencial contendo consultas estatísticas complexas em SQL (ex: alunos "carona", centros de reciclagem em destaque, médias de participação por região).

<br>

## 🎓 Equipe Acadêmica

Projeto desenvolvido para a disciplina **SCC0240 – Bases de Dados** do **Instituto de Ciências Matemáticas e de Computação (ICMC-USP)**.
- **Professora:** Elaine Parros Machado de Sousa
- **Monitor PAE:** Anderson Henrique Giacomini

| Participantes | Nº USP |
|---|---|
| 🧑‍💻 Pedro Henrique de Sousa Prestes | 15507819 |
| 👩‍💻 Laura Pazini Medeiros | 15468452 |
| 🧑‍💻 Fernando Valentim Torres | 15452340 |
| 🧑‍💻 Pedro Henrique Perez Dias | 15484075 |
| 🧑‍💻 Frederico Scheffel Oliveira | 15452718 |
