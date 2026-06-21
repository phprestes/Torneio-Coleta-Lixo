-- 1) Quais regiões coletaram a maior quantidade de lixo por categoria?

WITH Totais_Regiao AS (
    -- Calcula o peso total de cada categoria por região
    SELECT c.Lixo AS Categoria, p.Regiao AS Regiao, SUM(c.Peso) AS Total_Peso FROM Coleta c
	    INNER JOIN Ponto_de_Coleta pc ON c.Ponto_de_Coleta = pc.ID
	    INNER JOIN Partida p ON pc.Partida = p.ID
    GROUP BY c.Lixo, p.Regiao
),
Maximos_Categoria AS (
    -- Utiliza a tabela de peso total acima para descobrir o teto máximo de cada categoria
    SELECT Categoria, MAX(Total_Peso) AS Maximo_Peso FROM Totais_Regiao
    GROUP BY Categoria
)
-- Cruza as duas tabelas temporárias para exibir a região campeã por cada categoria
SELECT tr.Categoria, tr.Regiao, tr.Total_Peso FROM Totais_Regiao tr
	INNER JOIN Maximos_Categoria mc ON tr.Categoria = mc.Categoria AND tr.Total_Peso = mc.Maximo_Peso;

-- 2) Quantidade média de escolas que participam de cada tipo de campeonato ao longo dos anos?
SELECT Fase AS Tipo_Campeonato, ROUND(AVG(Qtd_Escolas)::NUMERIC, 2) AS Media_Escolas 
FROM (
    -- Subquery que calcula o total de participações por fase em cada ano
    SELECT p.Torneio AS Ano, p.Fase AS Fase, COUNT(a.Escola) AS Qtd_Escolas FROM Partida p
    	INNER JOIN Equipe_Participa_Partida ep ON p.ID = ep.Partida
    	INNER JOIN Aluno a ON ep.Nome_Equipe = a.Nome_Equipe AND ep.Ano_Equipe = a.Ano_Equipe
    GROUP BY p.Torneio, p.Fase
) AS Base_Anual
-- Ele junta os anos e tira a média de escolas de todas as fases ao longo de todos os anos
GROUP BY Fase;

-- 3) Faça um ranking de escola, pelo número de alunos MVP. Considerar as que não possuem MVPs e nem alunos cadastrados. 

SELECT
	-- Para mostrar o RANK de cada escola, foi feito essa parte do select usando o DENSE_RANK
	-- porém foi decidido que faz mais sentido aplicar esse rank no front da aplicação e não nessa consulta
    --CONCAT(DENSE_RANK() OVER (ORDER BY COUNT(p.AlunoMVP) DESC), 'º') AS Posicao_Ranking,
	e.ID AS Escola_ID, e.Nome AS Nome_Escola, COUNT(p.AlunoMVP) AS Total_MVP FROM Escola e
	-- Usa LEFT JOIN para garantir que todas as escolas apareçam no ranking, tendo alunos cadastrados ou não e tendo alunosMVP ou não
		LEFT JOIN Aluno a ON e.ID = a.Escola
		LEFT JOIN Partida p ON a.ID = p.AlunoMVP
GROUP BY e.ID, e.Nome
-- Ordena o resultado de forma decrescente com base na soma de MVPs
ORDER BY Total_MVP DESC;

-- 4) Quais os centros de reciclagem que receberam a maior quantidade de lixo por partida?

WITH Totais_Por_Centro AS (
    -- Calcula o peso total que cada centro de reciclagem recebeu por partida
    SELECT pc.Partida AS Partida_ID, cr.Nome AS Nome_Centro, SUM(c.Peso) AS Total_Recebido FROM Coleta c
	    INNER JOIN Ponto_de_Coleta pc ON c.Ponto_de_Coleta = pc.ID
	    INNER JOIN Logistica_Transporte lt ON pc.ID = lt.Ponto_de_Coleta
	    INNER JOIN Centro_de_Reciclagem cr ON lt.Centro_de_Reciclagem = cr.ID
    GROUP BY pc.Partida, cr.ID, cr.Nome
)
-- Relaciona a lista total pré-calculada com uma subquery que descobre qual foi o valor máximo obtido em cada partida
SELECT T.Partida_ID, T.Nome_Centro, T.Total_Recebido FROM Totais_Por_Centro T
	INNER JOIN (
	    SELECT Partida_ID, MAX(Total_Recebido) AS Max_Peso FROM Totais_Por_Centro
	    GROUP BY Partida_ID
	) Maximos ON T.Partida_ID = Maximos.Partida_ID 
  AND T.Total_Recebido = Maximos.Max_Peso;

-- 5) Quais são os alunos que participam de equipes que avançaram de fase, porém não coletaram nenhum lixo?

-- Seleciona o conjunto gerado pela subtração entre os conjuntos A - B
SELECT * FROM (
	-- CONJUNTO A: Alunos cujas equipes participaram de fases avançadas do torneio
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
