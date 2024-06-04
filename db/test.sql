SELECT b.identifier, b.navn, b.adresse
FROM behandler b
JOIN aabningstider oh ON b.identifier = oh.behandler_id
WHERE oh.day_of_week = strftime('%w', 'now', 'utc', '+4 hours')  
  AND time('now', 'utc', '+4 hours') BETWEEN oh.open_time AND oh.close_time;


WITH current_time AS (
    SELECT 
        strftime('%w', 'now', 'utc', '+4 hours') AS current_day_of_week, 
        time('now', 'utc', '+4 hours') AS current_time
),
next_opening AS (
    SELECT 
        b.identifier, 
        b.navn, 
        oh.day_of_week, 
        oh.open_time,
        CASE 
            WHEN oh.day_of_week = strftime('%w', 'now', 'utc', '+4 hours') AND oh.open_time > time('now', 'utc', '+4 hours') THEN 0
            WHEN oh.day_of_week > strftime('%w', 'now', 'utc', '+4 hours') THEN oh.day_of_week - strftime('%w', 'now')
            ELSE oh.day_of_week + 7 - strftime('%w', 'now', 'utc', '+4 hours')
        END AS days_until_open
    FROM 
        behandler b
    JOIN 
        aabningstider oh 
    ON 
        b.identifier = oh.behandler_id
)
SELECT 
    identifier, 
    navn, 
    day_of_week, 
    open_time
FROM 
    next_opening
ORDER BY 
    days_until_open, 
    open_time
LIMIT 1;
