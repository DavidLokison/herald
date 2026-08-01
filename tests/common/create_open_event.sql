INSERT INTO events (
    event_type_slug,
    title,
    start_date
)
VALUES (
    "hbz",
    "Test Event",
    DATE_ADD(CURRENT_DATE, INTERVAL 2 WEEK)
)
