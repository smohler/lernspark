--<< lernspark(job 1): Aggregate goals by city
INSERT INTO GoalsByCity
SELECT City, SUM(Goals) AS TotalGoals
FROM Football_teams
GROUP BY City
-->>
--<< lernspark(job 2): Find top 5 cities with the highest number of goals
INSERT INTO GoalsByCity
SELECT City, SUM(Goals) AS TotalGoals
FROM Football_teams
GROUP BY City
-->>
