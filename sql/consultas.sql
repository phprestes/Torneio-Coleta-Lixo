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