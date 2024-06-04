-- lernspark(job 1): Aggregate goals by city
INSERT INTO GoalsByCity
SELECT City, SUM(Goals) AS TotalGoals
FROM Football_Teams
GROUP BY City;
-- lernspark(job 2): Find top 5 cities with the highest number of goals
INSERT INTO Top5Cities
SELECT City, TotalGoals
FROM GoalsByCity
ORDER BY TotalGoals DESC
LIMIT 5;
-- lernspark(job N): <message>
