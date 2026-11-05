INSERT INTO events (
    event_type_slug,
    title,
    start_date
)
VALUES (
    "hbz",
    "Test Event",
    DATE_ADD(CURRENT_DATE, INTERVAL 2 WEEK)
);
SET @EventId=(SELECT event_id FROM events);
INSERT INTO registrations (
    event_id,
    name,
    address,
    email,
    phone,
    emergency_name,
    emergency_phone,
    comment
)
VALUES (
    @EventId,
    "Mister\tTesto\tTestoson",
    "Testroad 1\n01234 Sometown",
    "testo@example.com",
    "0123 4567890",
    "Testa Wifuwifu",
    "0123 4567891",
    ""
);
INSERT INTO persons (
    registration_id,
    name,
    birthday,
    address,
    comment,
    food_options
)
VALUES (
    @RegistrationId,
    "Mister\tTesto\tTestoson",
    DATE_SUB(CURRENT_DATE, INTERVAL 20 YEAR),
    "Testroad 1\n01234 Sometown",
    "",
    0
);
UPDATE registrations
SET status = (
    SELECT status
    FROM registration_statuses
    WHERE status_slug = "awaiting_approval"
)
WHERE registration_id = @RegistrationId;
