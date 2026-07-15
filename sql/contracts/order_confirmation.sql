SELECT events.title AS event_title, registrations.name AS registration_name, club_information.name_short AS club_sender, club_information.name_full AS club_name, registrations.status > 1 AS `approved: _`
FROM registrations
    JOIN events USING (event_id)
    CROSS JOIN club_information
WHERE registration_id = ?
