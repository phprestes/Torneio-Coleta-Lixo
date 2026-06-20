-- Quais regiões coletaram a maior quantidade de lixo por categoria?

-- Quantidade média de escolas que participam de cada tipo de campeonato ao longo dos anos?
SELECT Ano, Fase AS Tipo_Campeonato, AVG(Qtd_Escolas) AS Media_Escolas FROM (
    SELECT p.Torneio AS Ano, p.Fase AS Fase, COUNT(DISTINCT a.Escola) AS Qtd_Escolas FROM Partida p
		INNER JOIN Equipe_Participa_Partida ep ON p.ID = ep.Partida
	    INNER JOIN Aluno a ON ep.Nome_Equipe = a.Nome_Equipe AND ep.Ano_Equipe = a.Ano_Equipe
	    GROUP BY p.Torneio, p.Fase
) 
	GROUP BY Fase, Ano;

-- Faça um ranking de escola, pelo número de alunos MVP. Considerar as que não possuem MVPs e nem alunos cadastrados. 
SELECT e.ID AS Escola_ID, e.Nome AS Nome_Escola, COUNT(p.AlunoMVP) AS Total_MVP FROM Escola e
	LEFT JOIN Aluno a ON e.ID = a.Escola
	LEFT JOIN Partida p ON a.ID = p.AlunoMVP
	GROUP BY e.ID, e.Nome
	ORDER BY Total_MVP DESC;

-- Quais os centros de reciclagem que receberam a maior quantidade de lixo por partida?
 
-- Quais são os alunos que participam de equipes que avançaram de fase, porém não coletaram nenhum lixo?
-- CONJUNTO A: Alunos cujas equipes participaram de fases avançadas do torneio
SELECT * FROM (
	SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, a.Nome_Equipe FROM Aluno a
	INNER JOIN Escola e ON a.Escola = e.ID
	INNER JOIN Equipe_Participa_Partida ep ON a.Nome_Equipe = ep.Nome_Equipe AND a.Ano_Equipe = ep.Ano_Equipe
	INNER JOIN Partida p ON ep.Partida = p.ID
	WHERE UPPER(p.Fase) IN ('NACIONAL', 'CONTINENTAL', 'INTERNACIONAL')

	EXCEPT
	
	-- CONJUNTO B: Alunos que realizaram pelo menos UMA entrega de lixo (em qualquer momento)
	SELECT a.ID AS ID_Aluno, a.Nome AS Nome_Aluno, e.Nome AS Nome_Escola, a.Nome_Equipe FROM Aluno a
		INNER JOIN Escola e ON a.Escola = e.ID
		INNER JOIN Coleta c ON a.ID = c.Aluno
);