INSERT INTO Torneio (Ano, Tema, Porcentagem_Regional, Porcentagem_Nacional, Porcentagem_Continental, Porcentagem_Internacional) VALUES
(2024, 'Oceanos Sem Plástico', 20.00, 15.00, 10.00, 5.00),
(2025, 'Florestas e Reciclagem', 25.00, 20.00, 12.00, 6.00),
(2026, 'Cidades Lixo Zero', 30.00, 25.00, 15.00, 10.00);

INSERT INTO Escola (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
('CNPJ', '12345678000199', 'BRA', 'Escola Estadual Rodrigues Alves'),
('SIREN', '987654321', 'FRA', 'Lycée Louis-le-Grand'),
('EIN', '112233445', 'USA', 'Springfield High School'),
('CNPJ', '12388678110849', 'BRA', 'Colégio Objetivo - Unidade São Carlos');

INSERT INTO Lixo (Categoria, Pontuacao_KG) VALUES
('Plástico', 10.0),
('Alumínio', 25.5),
('Vidro', 5.0);

INSERT INTO Transportadora (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
(1, 'CNPJ', '11222333000100', 'BRA', 'TransResíduos Brasil'),
(2, 'SIREN', '555444333', 'FRA', 'EcoTrans Paris'),
(3, 'EIN', '998877665', 'USA', 'Global Hauling Inc');

INSERT INTO Centro_de_Reciclagem (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
(1, 'CNPJ', '44333222000111', 'BRA', 'Recicla São Carlos'),
(2, 'SIREN', '222111000', 'FRA', 'Centre de Tri Durable'),
(3, 'EIN', '444555666', 'USA', 'Green Cycle New York');

INSERT INTO Patrocinadores_Torneio (Torneio, Patrocinador) VALUES
(2024, 'Empresa EcoBr'),
(2025, 'Greenpeace Global'),
(2026, 'CleanEarth Inc');

INSERT INTO Tutor (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Cargo, Escola) VALUES
(1, 'CPF', '11122233344', 'BRA', 'Carlos Silva', '+5516999991111', 'Professor de Biologia', 1),
(2, 'CNI', '999888777', 'FRA', 'Marie Curie', '33144274427', 'Professeure de Chimie', 2),
(3, 'SSN', '555666777', 'USA', 'Walter White', '+15058425662', 'Chemistry Teacher', 3);

INSERT INTO Equipe (Nome, Ano, Tutor) VALUES
('EcoGuerreiros', 2024, 1),
('Les Écolos', 2025, 2),
('GreenTeam', 2026, 3);

INSERT INTO Aluno (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola) VALUES
(1, 'CPF', '44455566677', 'BRA', 'Pedro Henrique', '+5516988882222', 3, 'João Henrique', '+5516977773333', 1),
(2, 'CNI', '111444777', 'FRA', 'Laura Pazini', '33612345678', 2, 'Jean Pazini', '33687654321', 2),
(3, 'SSN', '222555888', 'USA', 'Fernando Torres', '+12025550143', 11, 'Maria Torres', '+12025550199', 3);

INSERT INTO Aluno_Equipe (Aluno, Nome_Equipe, Ano_Equipe) VALUES
(1, 'EcoGuerreiros', 2024),
(2, 'Les Écolos', 2025),
(3, 'GreenTeam', 2026);

INSERT INTO Partida (ID, Torneio, Fase, Regiao, Local_Partida, DataHora_Inicio, DataHora_Fim, AlunoMVP) VALUES
(1, 2024, 'REGIONAL', 'Sudeste BR', 'Bairro da Liberdade', '2024-05-10', '2024-05-12', 1),
(2, 2025, 'NACIONAL', 'França Centro', 'Paris Sul', '2025-06-15', '2025-06-15', 2),
(3, 2026, 'INTERNACIONAL', 'Mundial', 'São Carlos - SP', '2026-07-20', '2026-07-25', 3),
(4, 2025, 'REGIONAL', 'Sudeste BR', 'Bairro da Liberdade 2', '2024-05-10', '2025-05-12', 1);

INSERT INTO Equipe_Participa_Partida (Nome_Equipe, Ano_Equipe, Partida, Pontuacao) VALUES
('EcoGuerreiros', 2024, 1, 150.50),
('Les Écolos', 2025, 2, 210.00),
('GreenTeam', 2026, 3, 340.25);

INSERT INTO Ponto_de_Coleta (ID, Partida, Latitude, Longitude) VALUES
(1, 1, -23.561600, -46.638800),
(2, 2, 48.856600, 2.352200),
(3, 3, -22.008100, -47.890800);

INSERT INTO Coleta (Aluno, Ponto_de_Coleta, Data_Hora, Lixo, Peso, Pontuacao) VALUES
(1, 1, '2024-05-10 09:30:00', 'Plástico', 5.5, 55.0),
(2, 2, '2025-06-15 10:15:00', 'Alumínio', 2.0, 51.0),
(3, 3, '2026-07-20 11:00:00', 'Vidro', 10.0, 50.0);

INSERT INTO Logistica_Transporte (Ponto_de_Coleta, Transportadora, Centro_de_Reciclagem) VALUES
(1, 1, 1),
(2, 2, 2),
(3, 3, 3);

-- Inserindo alunos carona
INSERT INTO Aluno (ID, Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola) VALUES
(4, 'CNI', '111444888', 'FRA', 'Kylian Mbaguette', '33612345679', 2, 'Juliana Mbaguete', '33687654322', 2),

(5, 'SSN', '222555999', 'USA', 'Mac Donald', '+12025550144', 11, 'Donald Mac', '+12025550198', 3),

(6, 'CPF', '44455566688', 'BRA', 'Neymarzinho', '+5516988882223', 3, 'Neymar Idoso', '+5516977773334', 1);

INSERT INTO Aluno_Equipe (Aluno, Nome_Equipe, Ano_Equipe) VALUES
(4, 'Les Écolos', 2025),
(5, 'GreenTeam', 2026),
(6, 'EcoGuerreiros', 2024);
