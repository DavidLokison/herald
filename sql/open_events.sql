SELECT event_id AS `id: _`, event_type_slug AS type, title, begin, end AS `end!`, description
FROM api_events
WHERE deadline >= CURRENT_DATE
