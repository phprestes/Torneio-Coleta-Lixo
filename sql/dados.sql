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

INSERT INTO Transportadora (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
('CNPJ', '11222333000100', 'BRA', 'TransResíduos Brasil'),
('SIREN', '555444333', 'FRA', 'EcoTrans Paris'),
('EIN', '998877665', 'USA', 'Global Hauling Inc');

INSERT INTO Centro_de_Reciclagem (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
('CNPJ', '44333222000111', 'BRA', 'Recicla São Carlos'),
('SIREN', '222111000', 'FRA', 'Centre de Tri Durable'),
('EIN', '444555666', 'USA', 'Green Cycle New York');

INSERT INTO Patrocinadores_Torneio (Torneio, Patrocinador) VALUES
(2024, 'Empresa EcoBr'),
(2025, 'Greenpeace Global'),
(2026, 'CleanEarth Inc');

INSERT INTO Tutor (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Cargo, Escola) VALUES
('CPF', '11122233344', 'BRA', 'Carlos Silva', '+5516999991111', 'Professor de Biologia', 1),
('CNI', '999888777', 'FRA', 'Marie Curie', '33144274427', 'Professeure de Chimie', 2),
('SSN', '555666777', 'USA', 'Walter White', '+15058425662', 'Chemistry Teacher', 3);

INSERT INTO Equipe (Nome, Ano, Tutor) VALUES
('EcoGuerreiros', 2024, 1),
('Les Écolos', 2025, 2),
('GreenTeam', 2026, 3);

INSERT INTO Aluno (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola) VALUES
('CPF', '44455566677', 'BRA', 'Pedro Henrique', '+5516988882222', 3, 'João Henrique', '+5516977773333', 1),
('CNI', '111444777', 'FRA', 'Laura Pazini', '33612345678', 2, 'Jean Pazini', '33687654321', 2),
('SSN', '222555888', 'USA', 'Fernando Torres', '+12025550143', 11, 'Maria Torres', '+12025550199', 3);

INSERT INTO Aluno_Equipe (Aluno, Nome_Equipe, Ano_Equipe) VALUES
(1, 'EcoGuerreiros', 2024),
(2, 'Les Écolos', 2025),
(3, 'GreenTeam', 2026);

INSERT INTO Partida (Torneio, Fase, Regiao, Local_Partida, DataHora_Inicio, DataHora_Fim, AlunoMVP) VALUES
(2024, 'REGIONAL', 'Sudeste BR', 'Bairro da Liberdade', '2024-05-10', '2024-05-12', 1),
(2025, 'NACIONAL', 'França', 'Paris Sul', '2025-06-15', '2025-06-15', 2),
(2026, 'INTERNACIONAL', 'Mundial', 'São Carlos - SP', '2026-07-20', '2026-07-25', 3),
(2025, 'REGIONAL', 'Sudeste BR', 'Bairro da Liberdade 2', '2024-05-10', '2025-05-12', 1);

INSERT INTO Equipe_Participa_Partida (Nome_Equipe, Ano_Equipe, Partida, Pontuacao) VALUES
('EcoGuerreiros', 2024, 1, 150.50),
('Les Écolos', 2025, 2, 210.00),
('GreenTeam', 2026, 3, 340.25);

INSERT INTO Ponto_de_Coleta (Partida, Latitude, Longitude) VALUES
(1, -23.561600, -46.638800),
(2, 48.856600, 2.352200),
(3, -22.008100, -47.890800);

INSERT INTO Coleta (Aluno, Ponto_de_Coleta, Data_Hora, Lixo, Peso, Pontuacao) VALUES
(1, 1, '2024-05-10 09:30:00', 'Plástico', 5.5, 55.0),
(2, 2, '2025-06-15 10:15:00', 'Alumínio', 2.0, 51.0),
(3, 3, '2026-07-20 11:00:00', 'Vidro', 10.0, 50.0);

INSERT INTO Logistica_Transporte (Ponto_de_Coleta, Transportadora, Centro_de_Reciclagem) VALUES
(1, 1, 1),
(2, 2, 2),
(3, 3, 3);

-- Inserindo alunos carona
INSERT INTO Aluno (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola) VALUES
('CNI', '111444888', 'FRA', 'Kylian Mbaguette', '33612345679', 2, 'Juliana Mbaguete', '33687654322', 2),

('SSN', '222555999', 'USA', 'Mac Donald', '+12025550144', 11, 'Donald Mac', '+12025550198', 3),

('CPF', '44455566688', 'BRA', 'Neymarzinho', '+5516988882223', 3, 'Neymar Idoso', '+5516977773334', 1);

INSERT INTO Aluno_Equipe (Aluno, Nome_Equipe, Ano_Equipe) VALUES
(4, 'Les Écolos', 2025),
(5, 'GreenTeam', 2026),
(6, 'EcoGuerreiros', 2024);

-- ==========================================
-- DADOS PARA CASOS ESPECIAIS (TESTES GERAIS)
-- ==========================================

-- Adicionando mais escolas (receberão IDs 5 e 6 devido ao SERIAL)
INSERT INTO Escola (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome) VALUES
('CNPJ', '11111111000199', 'BRA', 'Colégio Teste Alpha'),
('CNPJ', '22222222000199', 'BRA', 'Colégio Teste Beta');

-- Adicionando mais tutores
INSERT INTO Tutor (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Cargo, Escola) VALUES
('CPF', '44444444444', 'BRA', 'Tutor Alpha', '+5516999999999', 'Professor', 5),
('CPF', '55555555555', 'BRA', 'Tutor Beta', '+5516988888888', 'Professor', 6);

-- Adicionando Múltiplas Equipes para o mesmo torneio
INSERT INTO Equipe (Nome, Ano, Tutor) VALUES
('Time Alpha', 2026, 4),
('Time Beta', 2026, 5);

-- Adicionando Alunos para essas novas equipes
INSERT INTO Aluno (Tipo_Documento, Numero_Documento, Sigla_Pais, Nome, Contato, Serie, Nome_Responsavel, Contato_Responsavel, Escola) VALUES
('CPF', '77777777777', 'BRA', 'Aluno Alpha 1', '+5511', 1, 'Resp Alpha 1', '+5511', 5),
('CPF', '88888888888', 'BRA', 'Aluno Alpha 2', '+5522', 1, 'Resp Alpha 2', '+5522', 5),
('CPF', '99999999999', 'BRA', 'Aluno Beta 1', '+5533', 1, 'Resp Beta 1', '+5533', 6);

INSERT INTO Aluno_Equipe (Aluno, Nome_Equipe, Ano_Equipe) VALUES
(7, 'Time Alpha', 2026),
(8, 'Time Alpha', 2026),
(9, 'Time Beta', 2026);

-- Partida ocorrendo AGORA (Gordura de 15 dias antes até 15 dias depois da data de hoje)
INSERT INTO Partida (Torneio, Fase, Regiao, Local_Partida, DataHora_Inicio, DataHora_Fim, AlunoMVP) VALUES
(2026, 'NACIONAL', 'Sul BR', 'Florianópolis - SC', CURRENT_DATE - 15, CURRENT_DATE + 15, NULL);

-- Inserindo as duas equipes na mesma partida (Partida 5)
INSERT INTO Equipe_Participa_Partida (Nome_Equipe, Ano_Equipe, Partida, Pontuacao) VALUES
('Time Alpha', 2026, 5, 20.0),
('Time Beta', 2026, 5, 25.0);

-- Ponto de coleta exclusivo para a partida atual
INSERT INTO Ponto_de_Coleta (Partida, Latitude, Longitude) VALUES
(5, -27.595400, -48.548000);

-- Logística para o novo ponto
INSERT INTO Logistica_Transporte (Ponto_de_Coleta, Transportadora, Centro_de_Reciclagem) VALUES
(4, 1, 1);

-- Algumas coletas na partida atual para refletir os pontos mockados
INSERT INTO Coleta (Aluno, Ponto_de_Coleta, Data_Hora, Lixo, Peso, Pontuacao) VALUES
(7, 4, CURRENT_TIMESTAMP, 'Plástico', 2.0, 20.0),
(9, 4, CURRENT_TIMESTAMP, 'Vidro', 5.0, 25.0);
