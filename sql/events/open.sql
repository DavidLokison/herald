SELECT event_id AS `id: _`, event_type_slug AS type, title, start_date AS start, DATE_ADD(start_date, INTERVAL duration DAY) AS `end!`, description
FROM EVENTS
    INNER JOIN event_types USING (event_type_slug)
WHERE DATE_SUB(start_date, INTERVAL deadline_registration DAY) >= CURRENT_DATE
