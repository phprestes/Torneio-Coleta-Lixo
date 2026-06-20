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