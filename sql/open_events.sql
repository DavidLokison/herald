SELECT event_id AS `id: _`, event_type_slug AS type, title, begin_date AS begin, DATE_ADD(begin_date, INTERVAL duration DAY) AS `end!`, description
FROM EVENTS
    INNER JOIN event_types USING (event_type_slug)
WHERE DATE_SUB(begin_date, INTERVAL deadline_registration DAY) >= CURRENT_DATE
