SELECT
    events.title AS event_title,
    JSON_OBJECT(
        'name', registrations.name,
        'email', registrations.email
    ) AS `recipient: Json<Contact>`,
    JSON_OBJECT(
        'short', club_information.name_short,
        'long', club_information.name_full,
        'email', club_information.email
    ) AS `club: Json<ClubInformation>`,
    JSON_OBJECT(
        'value', 0,
        'decimals', club_information.bank_currency_decimals,
        'currency', club_information.bank_currency
    ) AS `total: Json<Price>`,
    JSON_OBJECT(
        'value', 0,
        'decimals', club_information.bank_currency_decimals,
        'currency', club_information.bank_currency
    ) AS `paid: Json<Price>`
FROM registrations
    JOIN events USING (event_id)
    CROSS JOIN club_information
WHERE registration_id = ?
