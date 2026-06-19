-- Tabelas sem FK

CREATE TABLE Torneio (
    Ano NUMBER NOT NULL,
    Tema VARCHAR2(30),
    Porcentagem_Regional NUMBER NOT NULL,
    Porcentagem_Nacional NUMBER NOT NULL,
    Porcentagem_Continental NUMBER NOT NULL,
    Porcentagem_Internacional NUMBER NOT NULL,
    
    CONSTRAINT PK_Torneio PRIMARY KEY (Ano)
);

CREATE TABLE Escola (
    ID NUMBER NOT NULL,
    Tipo_Documento VARCHAR2(10) NOT NULL,
    Numero_Documento VARCHAR2(20) NOT NULL,
    Sigla_Pais CHAR(2) NOT NULL,
    Nome VARCHAR2(255) NOT NULL,
    
    CONSTRAINT PK_Escola PRIMARY KEY (ID),
    CONSTRAINT UK_Escola UNIQUE (Tipo_Documento, Numero_Documento, Sigla_Pais)
);

CREATE TABLE Lixo (
    Categoria VARCHAR2(30) NOT NULL,
    Pontuacao_KG NUMBER NOT NULL,
    
    CONSTRAINT PK_Lixo PRIMARY KEY (Categoria)
);

CREATE TABLE Transportadora (
    ID NUMBER NOT NULL,
    Tipo_Documento VARCHAR2(10) NOT NULL,
    Numero_Documento VARCHAR2(20) NOT NULL,
    Sigla_Pais CHAR(2) NOT NULL,
    Nome VARCHAR2(255),
    
    CONSTRAINT PK_Transportadora PRIMARY KEY (ID),
    CONSTRAINT UK_Transportadora UNIQUE (Tipo_Documento, Numero_Documento, Sigla_Pais)
);

CREATE TABLE Centro_de_Reciclagem (
    ID NUMBER NOT NULL,
    Tipo_Documento VARCHAR2(10) NOT NULL,
    Numero_Documento VARCHAR2(20) NOT NULL,
    Sigla_Pais CHAR(2) NOT NULL,
    Nome VARCHAR2(255),
    
    CONSTRAINT PK_Centro_de_Reciclagem PRIMARY KEY (ID),
    CONSTRAINT UK_Centro_de_Reciclagem UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais)
);

-- Tabelas com FK

CREATE TABLE Patrocinadores_Torneio (
    Torneio NUMBER NOT NULL,
    Patrocinador VARCHAR2(255) NOT NULL,
    
    CONSTRAINT PK_Patrocinadores PRIMARY KEY (Torneio, Patrocinador),
    CONSTRAINT FK_Patrocinadores FOREIGN KEY (Torneio)
        REFERENCES Torneio(Ano)
        ON DELETE CASCADE
);

CREATE TABLE Tutor (
    ID NUMBER NOT NULL,
    Tipo_Documento VARCHAR2(10) NOT NULL,
    Numero_Documento VARCHAR2(20) NOT NULL,
    Sigla_Pais CHAR(2) NOT NULL,
    Nome VARCHAR2(255) NOT NULL,
    Contato NUMBER NOT NULL,
    Cargo VARCHAR2(30),
    Escola NUMBER NOT NULL,
    
    CONSTRAINT PK_Tutor PRIMARY KEY (ID),
    CONSTRAINT UK_Tutor UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais),
    CONSTRAINT FK_Tutor FOREIGN KEY (Escola)
        REFERENCES Escola(ID)
        ON DELETE CASCADE
);

CREATE TABLE Equipe (
    Nome VARCHAR2(255) NOT NULL,
    Ano NUMBER NOT NULL,
    Tutor NUMBER NOT NULL,
    
    CONSTRAINT PK_Equipe PRIMARY KEY (Nome, Ano),
    CONSTRAINT FK_Equipe FOREIGN KEY (Tutor)
        REFERENCES Tutor(ID)
        ON DELETE CASCADE
);

CREATE TABLE Aluno (
    ID NUMBER NOT NULL,
    Tipo_Documento VARCHAR2(10) NOT NULL,
    Numero_Documento VARCHAR2(20) NOT NULL,
    Sigla_Pais CHAR(2) NOT NULL,
    Nome VARCHAR2(255) NOT NULL,
    Contato NUMBER,
    Serie NUMBER NOT NULL,
    Nome_Responsavel VARCHAR2(255),
    Contato_Responsavel NUMBER,
    Escola NUMBER NOT NULL,
    Nome_Equipe VARCHAR2(255) NOT NULL,
    Ano_Equipe NUMBER NOT NULL,
    
    CONSTRAINT PK_Aluno PRIMARY KEY (ID),
    CONSTRAINT UK_Aluno UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais),
    CONSTRAINT FK_Aluno_Escola FOREIGN KEY (Escola)
        REFERENCES Escola(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Aluno_Equipe FOREIGN KEY (Nome_Equipe, Ano_Equipe)
        REFERENCES Equipe(Nome, Ano)
        ON DELETE CASCADE
);

CREATE TABLE Partida (
    ID NUMBER NOT NULL,
    Torneio NUMBER NOT NULL,
    Fase VARCHAR2(13) NOT NULL,
    Regiao VARCHAR2(255) NOT NULL,
    Local_Partida VARCHAR2(255) NOT NULL,
    DT_Inicio DATE NOT NULL,
    DT_Fim DATE NOT NULL,
    AlunoMVP NUMBER,
    
    CONSTRAINT PK_Partida PRIMARY KEY (ID),
    CONSTRAINT UK_Partida UNIQUE(Torneio, Fase, Regiao, Local_Partida),
    CONSTRAINT FK_Partida_Torneio FOREIGN KEY (Torneio)
        REFERENCES Torneio(Ano)
        ON DELETE CASCADE,
    CONSTRAINT FK_Partida_MVP FOREIGN KEY (AlunoMVP)
        REFERENCES Aluno(ID)
        ON DELETE SET NULL,
    CONSTRAINT CK_Fase CHECK(UPPER(Fase) IN ('REGIONAL', 'NACIONAL', 'CONTINENTAL', 'INTERNACIONAL'))
);

CREATE TABLE Equipe_Participa_Partida (
    Nome_Equipe VARCHAR2(255) NOT NULL,
    Ano_Equipe NUMBER NOT NULL,
    Partida NUMBER NOT NULL,
    Pontuacao NUMBER DEFAULT 0 NOT NULL,
    
    CONSTRAINT PK_Equipe_Participa_Partida PRIMARY KEY (Nome_Equipe, Ano_Equipe, Partida),
    CONSTRAINT FK_EQUIPE_PARTICIPA FOREIGN KEY (Nome_Equipe, Ano_Equipe)
        REFERENCES Equipe(Nome, Ano)
        ON DELETE CASCADE,
    CONSTRAINT FK_Participa_Partida FOREIGN KEY (Partida)
        REFERENCES Partida(ID)
        ON DELETE CASCADE
);

CREATE TABLE Ponto_de_Coleta (
    ID NUMBER NOT NULL,
    Partida NUMBER NOT NULL,
    Latitude NUMBER(10, 6) NOT NULL,
    Longitude NUMBER(10, 6) NOT NULL,
    
    CONSTRAINT PK_Ponto_de_Coleta PRIMARY KEY (ID),
    CONSTRAINT UK_Ponto_de_Coleta UNIQUE(Partida, Latitude, Longitude),
    CONSTRAINT FK_Ponto_de_Coleta FOREIGN KEY (Partida)
        REFERENCES Partida(ID)
        ON DELETE CASCADE
);

CREATE TABLE Coleta (
    Aluno NUMBER NOT NULL,
    Ponto_de_Coleta NUMBER NOT NULL,
    Data_Hora DATE NOT NULL,
    Lixo VARCHAR2(30) NOT NULL,
    Peso NUMBER NOT NULL,
    Pontuacao NUMBER NOT NULL,
    
    CONSTRAINT PK_Coleta PRIMARY KEY (Aluno, Ponto_de_Coleta, Data_Hora),
    CONSTRAINT FK_Coleta_Ponto FOREIGN KEY (Ponto_de_Coleta)
        REFERENCES Ponto_de_Coleta(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Coleta_Lixo FOREIGN KEY (Lixo)
        REFERENCES Lixo(Categoria)
        ON DELETE CASCADE
);

CREATE TABLE Logistica_Transporte (
    Ponto_de_Coleta NUMBER NOT NULL,
    Transportadora NUMBER NOT NULL,
    Centro_de_Reciclagem NUMBER NOT NULL,
    
    CONSTRAINT PK_Logistica PRIMARY KEY (Ponto_de_Coleta, Transportadora, Centro_de_Reciclagem),
    CONSTRAINT FK_Logistica_Ponto FOREIGN KEY (Ponto_de_Coleta)
        REFERENCES Ponto_de_Coleta(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Logistica_Transportadora FOREIGN KEY (Transportadora)
        REFERENCES Transportadora(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Logistica_Centro FOREIGN KEY (Centro_de_Reciclagem)
        REFERENCES Centro_de_Reciclagem(ID)
        ON DELETE CASCADE
);
