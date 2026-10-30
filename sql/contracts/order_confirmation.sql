SELECT events.title, registrations.name AS registration_name, registrations.email AS registration_email, club_information.name_short AS club_name_short, club_information.name_full AS club_name_full, club_information.email AS club_email, registrations.status > 1 AS `autoapproved: _`
FROM registrations
    JOIN events USING (event_id)
    CROSS JOIN club_information
WHERE registration_id = ?
