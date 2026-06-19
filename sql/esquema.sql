-- DOMÍNIOS
CREATE DOMAIN SIGLA_ALFA_3 AS CHAR(3)
CHECK (VALUE ~ '^[A-Z]{3}$');

-- TABELAS SEM CHAVE ESTRANGEIRA (FK)
CREATE TABLE Torneio (
    Ano INTEGER NOT NULL,
    Tema VARCHAR(30),
    Porcentagem_Regional NUMERIC NOT NULL,
    Porcentagem_Nacional NUMERIC NOT NULL,
    Porcentagem_Continental NUMERIC NOT NULL,
    Porcentagem_Internacional NUMERIC NOT NULL,
    
    CONSTRAINT PK_Torneio PRIMARY KEY (Ano),
    CONSTRAINT CK_Porcentagem CHECK(
        (Porcentagem_Regional BETWEEN 0 AND 100) AND 
        (Porcentagem_Nacional BETWEEN 0 AND 100) AND
        (Porcentagem_Continental BETWEEN 0 AND 100) AND
        (Porcentagem_Internacional BETWEEN 0 AND 100)
    ),
    CONSTRAINT CK_Ano CHECK (Ano >= 1900)
);

CREATE TABLE Escola (
    ID INTEGER NOT NULL,
    Tipo_Documento VARCHAR(10) NOT NULL,
    Numero_Documento VARCHAR(20) NOT NULL,
    Sigla_Pais SIGLA_ALFA_3 NOT NULL,
    Nome VARCHAR(255) NOT NULL,
    
    CONSTRAINT PK_Escola PRIMARY KEY (ID),
    CONSTRAINT UK_Escola UNIQUE (Tipo_Documento, Numero_Documento, Sigla_Pais)
);

CREATE TABLE Lixo (
    Categoria VARCHAR(30) NOT NULL,
    Pontuacao_KG NUMERIC NOT NULL,
    
    CONSTRAINT PK_Lixo PRIMARY KEY (Categoria),
    CONSTRAINT CK_Pontuacao_Lixo CHECK (Pontuacao_KG >= 0)
);

CREATE TABLE Transportadora (
    ID INTEGER NOT NULL,
    Tipo_Documento VARCHAR(10) NOT NULL,
    Numero_Documento VARCHAR(20) NOT NULL,
    Sigla_Pais SIGLA_ALFA_3 NOT NULL,
    Nome VARCHAR(255),
    
    CONSTRAINT PK_Transportadora PRIMARY KEY (ID),
    CONSTRAINT UK_Transportadora UNIQUE (Tipo_Documento, Numero_Documento, Sigla_Pais)
);

CREATE TABLE Centro_de_Reciclagem (
    ID INTEGER NOT NULL,
    Tipo_Documento VARCHAR(10) NOT NULL,
    Numero_Documento VARCHAR(20) NOT NULL,
    Sigla_Pais SIGLA_ALFA_3 NOT NULL,
    Nome VARCHAR(255),
    
    CONSTRAINT PK_Centro_de_Reciclagem PRIMARY KEY (ID),
    CONSTRAINT UK_Centro_de_Reciclagem UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais)
);

-- TABELAS COM CHAVE ESTRANGEIRA (FK)
CREATE TABLE Patrocinadores_Torneio (
    Torneio INTEGER NOT NULL,
    Patrocinador VARCHAR(255) NOT NULL,
    
    CONSTRAINT PK_Patrocinadores PRIMARY KEY (Torneio, Patrocinador),
    CONSTRAINT FK_Patrocinadores FOREIGN KEY (Torneio)
        REFERENCES Torneio(Ano)
        ON DELETE CASCADE
);

CREATE TABLE Tutor (
    ID INTEGER NOT NULL,
    Tipo_Documento VARCHAR(10) NOT NULL,
    Numero_Documento VARCHAR(20) NOT NULL,
    Sigla_Pais SIGLA_ALFA_3 NOT NULL,
    Nome VARCHAR(255) NOT NULL,
    Contato VARCHAR(15) NOT NULL,
    Cargo VARCHAR(30),
    Escola INTEGER NOT NULL,
    
    CONSTRAINT PK_Tutor PRIMARY KEY (ID),
    CONSTRAINT UK_Tutor UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais),
    CONSTRAINT FK_Tutor FOREIGN KEY (Escola)
        REFERENCES Escola(ID)
        ON DELETE CASCADE,
    CONSTRAINT CK_Tutor_Contato CHECK (Contato ~ '^\+?[0-9]{8,15}$') 
);

CREATE TABLE Equipe (
    Nome VARCHAR(255) NOT NULL,
    Ano INTEGER NOT NULL,
    Tutor INTEGER NOT NULL,
    
    CONSTRAINT PK_Equipe PRIMARY KEY (Nome, Ano),
    CONSTRAINT FK_Equipe FOREIGN KEY (Tutor)
        REFERENCES Tutor(ID)
        ON DELETE CASCADE
);

CREATE TABLE Aluno (
    ID INTEGER NOT NULL,
    Tipo_Documento VARCHAR(10) NOT NULL,
    Numero_Documento VARCHAR(20) NOT NULL,
    Sigla_Pais SIGLA_ALFA_3 NOT NULL,
    Nome VARCHAR(255) NOT NULL,
    Contato VARCHAR(15), 
    Serie INTEGER NOT NULL,
    Nome_Responsavel VARCHAR(255),
    Contato_Responsavel VARCHAR(15), 
    Escola INTEGER NOT NULL,
    Nome_Equipe VARCHAR(255) NOT NULL,
    Ano_Equipe INTEGER NOT NULL,
    
    CONSTRAINT PK_Aluno PRIMARY KEY (ID),
    CONSTRAINT UK_Aluno UNIQUE(Tipo_Documento, Numero_Documento, Sigla_Pais),
    CONSTRAINT FK_Aluno_Escola FOREIGN KEY (Escola)
        REFERENCES Escola(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Aluno_Equipe FOREIGN KEY (Nome_Equipe, Ano_Equipe)
        REFERENCES Equipe(Nome, Ano)
        ON DELETE CASCADE,
    CONSTRAINT CK_Aluno_Contato CHECK (Contato IS NULL OR Contato ~ '^\+?[0-9]{8,15}$'),
    CONSTRAINT CK_Resp_Contato CHECK (Contato_Responsavel IS NULL OR Contato_Responsavel ~ '^\+?[0-9]{8,15}$')
);

CREATE TABLE Partida (
    ID INTEGER NOT NULL,
    Torneio INTEGER NOT NULL,
    Fase VARCHAR(13) NOT NULL,
    Regiao VARCHAR(255) NOT NULL,
    Local_Partida VARCHAR(255) NOT NULL,
    DataHora_Inicio DATE NOT NULL,
    DataHora_Fim DATE NOT NULL,
    AlunoMVP INTEGER,
    
    CONSTRAINT PK_Partida PRIMARY KEY (ID),
    -- VERIFICAR: CONSTRAINT UK_Partida UNIQUE(Torneio, Fase, Regiao, Local_Partida),
    CONSTRAINT UK_Partida UNIQUE(Torneio, Local_Partida),
    CONSTRAINT FK_Partida_Torneio FOREIGN KEY (Torneio)
        REFERENCES Torneio(Ano)
        ON DELETE CASCADE,
    CONSTRAINT FK_Partida_MVP FOREIGN KEY (AlunoMVP)
        REFERENCES Aluno(ID)
        ON DELETE SET NULL,
    CONSTRAINT CK_Fase CHECK(UPPER(Fase) IN ('REGIONAL', 'NACIONAL', 'CONTINENTAL', 'INTERNACIONAL')),
    CONSTRAINT CK_Datas_Partida CHECK (DataHora_Fim >= DataHora_Inicio) 
);

CREATE TABLE Equipe_Participa_Partida (
    Nome_Equipe VARCHAR(255) NOT NULL,
    Ano_Equipe INTEGER NOT NULL,
    Partida INTEGER NOT NULL,
    Pontuacao NUMERIC DEFAULT 0 NOT NULL,
    
    CONSTRAINT PK_Equipe_Participa_Partida PRIMARY KEY (Nome_Equipe, Ano_Equipe, Partida),
    CONSTRAINT FK_EQUIPE_PARTICIPA FOREIGN KEY (Nome_Equipe, Ano_Equipe)
        REFERENCES Equipe(Nome, Ano)
        ON DELETE CASCADE,
    CONSTRAINT FK_Participa_Partida FOREIGN KEY (Partida)
        REFERENCES Partida(ID)
        ON DELETE CASCADE,
    CONSTRAINT CK_Pontuacao_Minima CHECK (Pontuacao >= 0)
);

CREATE TABLE Ponto_de_Coleta (
    ID INTEGER NOT NULL,
    Partida INTEGER NOT NULL,
    Latitude NUMERIC(10, 6) NOT NULL,
    Longitude NUMERIC(10, 6) NOT NULL,
    
    CONSTRAINT PK_Ponto_de_Coleta PRIMARY KEY (ID),
    CONSTRAINT UK_Ponto_de_Coleta UNIQUE(Partida, Latitude, Longitude),
    CONSTRAINT FK_Ponto_de_Coleta FOREIGN KEY (Partida)
        REFERENCES Partida(ID)
        ON DELETE CASCADE,
    CONSTRAINT CK_Latitude CHECK (Latitude BETWEEN -90 AND 90),
    CONSTRAINT CK_Longitude CHECK (Longitude BETWEEN -180 AND 180) 
);

CREATE TABLE Coleta (
    Aluno INTEGER NOT NULL,
    Ponto_de_Coleta INTEGER NOT NULL,
    Data_Hora TIMESTAMP NOT NULL,
    Lixo VARCHAR(30) NOT NULL,
    Peso NUMERIC NOT NULL,
    Pontuacao NUMERIC NOT NULL,
    
    CONSTRAINT PK_Coleta PRIMARY KEY (Aluno, Ponto_de_Coleta, Data_Hora),
    CONSTRAINT FK_Coleta_Aluno FOREIGN KEY (Aluno)
        REFERENCES Aluno(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Coleta_Ponto FOREIGN KEY (Ponto_de_Coleta)
        REFERENCES Ponto_de_Coleta(ID)
        ON DELETE CASCADE,
    CONSTRAINT FK_Coleta_Lixo FOREIGN KEY (Lixo)
        REFERENCES Lixo(Categoria)
        ON DELETE CASCADE,
    CONSTRAINT CK_Coleta_Peso CHECK (Peso > 0)
);

CREATE TABLE Logistica_Transporte (
    Ponto_de_Coleta INTEGER NOT NULL,
    Transportadora INTEGER NOT NULL,
    Centro_de_Reciclagem INTEGER NOT NULL,
    
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
)

-- INSERTS
-- 1. Tabelas sem Chaves Estrangeiras
-- ---------------------------------------------------------

INSERT INTO Torneio (Ano, Tema, Porcentagem_Regional, Porcentagem_Nacional, Porcentagem_Continental, Porcentagem_Internacional) VALUES
(2024, 'Oceanos Limpos', 40.0, 30.0, 20.0, 10.0),
(2025, 'Cidades Sustentáveis', 50.0, 25.0, 15.0, 10.0);

INSERT INTO Lixo (Categoria, Pontuacao_KG) VALUES
('Plástico', 10.5),
('Vidro', 5.0),
('Metal', 15.0),
('Papel', 3.5),
('Eletrônico', 25.0);

INSERT INTO Escola (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
(1, 'CNPJ', '12345678000199', 'BRA', 'Escola Estadual São Carlos'),
(2, 'CNPJ', '98765432000188', 'BRA', 'Colégio Bandeirantes');

INSERT INTO Transportadora (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
(1, 'CNPJ', '11122233000144', 'BRA', 'EcoTrans Logística'),
(2, 'CNPJ', '55566677000188', 'BRA', 'Logística Verde S/A');

INSERT INTO Centro_de_Reciclagem (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
(1, 'CNPJ', '99988877000166', 'BRA', 'Recicla Tudo Centro-Sul'),
(2, 'CNPJ', '44433322000111', 'BRA', 'Centro de Tratamento Sustentável SP');


-- 2. Tabelas com Chaves Estrangeiras
-- ---------------------------------------------------------

INSERT INTO Patrocinadores_Torneio (Torneio, Patrocinador) VALUES
(2024, 'Instituto Natureza'),
(2024, 'Tech para o Bem'),
(2025, 'Fundo Global do Meio Ambiente');

INSERT INTO Tutor (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Cargo, Escola) VALUES
(1, 'CPF', '11122233344', 'BRA', 'João Silva', '+5516999999999', 'Professor de Biologia', 1),
(2, 'CPF', '55566677788', 'BRA', 'Maria Oliveira', '+5511988888888', 'Coordenadora de Ciências', 2);

INSERT INTO Equipe (Nome, Ano, Tutor) VALUES
('EcoWarriors', 2024, 1),
('GreenFuture', 2025, 2);

INSERT INTO Aluno (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola, Nome_Equipe, Ano_Equipe) VALUES
(1, 'RG', '123456789', 'BRA', 'Pedro Santos', '+5516977777777', 9, 'Ana Santos', '+5516966666666', 1, 'EcoWarriors', 2024),
(2, 'RG', '987654321', 'BRA', 'Lucas Alves', '+5511955555555', 1, 'Carlos Alves', '+5511944444444', 2, 'GreenFuture', 2025),
(3, 'RG', '456789123', 'BRA', 'Beatriz Costa', '+5516933333333', 9, 'Marcos Costa', '+5516922222222', 1, 'EcoWarriors', 2024);

-- O MVP deve ser inserido agora, pois já temos os Alunos cadastrados.
INSERT INTO Partida (ID, Torneio, Fase, Regiao, Local_Partida, DataHora_Inicio, DataHora_Fim, AlunoMVP) VALUES
(1, 2024, 'REGIONAL', 'Sudeste', 'Praça Central de São Carlos', '2024-05-10', '2024-05-10', 1),
(2, 2025, 'NACIONAL', 'Brasil', 'Parque Ibirapuera SP', '2025-08-20', '2025-08-21', 2);

INSERT INTO Equipe_Participa_Partida (Nome_Equipe, Ano_Equipe, Partida, Pontuacao) VALUES
('EcoWarriors', 2024, 1, 1500.50),
('GreenFuture', 2025, 2, 2300.00);

-- Latitudes e Longitudes do estado de São Paulo (respeitando o CK)
INSERT INTO Ponto_de_Coleta (ID, Partida, Latitude, Longitude) VALUES
(1, 1, -22.015400, -47.891100), 
(2, 2, -23.587400, -46.657600);

-- Calculando pontuação fictícia: Peso * Pontuacao_KG da tabela Lixo
INSERT INTO Coleta (Aluno, Ponto_de_Coleta, Data_Hora, Lixo, Peso, Pontuacao) VALUES
(1, 1, '2024-05-10 14:30:00', 'Plástico', 10.0, 105.0), 
(3, 1, '2024-05-10 15:45:00', 'Metal', 2.0, 30.0),
(2, 2, '2025-08-20 10:15:00', 'Eletrônico', 5.0, 125.0);

INSERT INTO Logistica_Transporte (Ponto_de_Coleta, Transportadora, Centro_de_Reciclagem) VALUES
(1, 1, 1),
(2, 2, 2);

-- CONSULTAS:
-- Quais regiões coletaram a maior quantidade de lixo por categoria?
SELECT MAX("Lixo/Categoria") FROM (
	SELECT p.Regiao, l.Categoria, COUNT(*) as "Lixo/Categoria" FROM Lixo l
		JOIN Coleta c ON c.Lixo = l.categoria
		JOIN Partida p ON c.Aluno = p.AlunoMVP
		GROUP BY p.Regiao, l.Categoria
) 
	
	
-- Quantidade média de escolas que participam de cada tipo de campeonato ao longo dos anos?
-- Qual a escola que teve o maior número de alunos MVP (GOATs)?
-- Quais os centros de reciclagem que receberam a maior quantidade de lixo por partida?
-- Quais são os alunos que participam de equipes que avançaram de fase, porém não coletaram nenhum lixo?