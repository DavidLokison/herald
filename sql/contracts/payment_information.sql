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
    registrations.invoice_id AS `invoice_id: &[u8]`,
    JSON_ARRAYAGG(
        JSON_OBJECT(
            'id', HEX(articles.article_id),
            'description', articles.description,
            'count', registration_invoices.count,
            'price', JSON_OBJECT(
                'value', articles.price,
                'decimals', club_information.bank_currency_decimals,
                'currency', club_information.bank_currency
            )
        )
    ) AS `articles!: Articles`,
    JSON_OBJECT(
        'value', SUM(registration_invoices.count * articles.price),
        'decimals', club_information.bank_currency_decimals,
        'currency', club_information.bank_currency
    ) AS `total: Json<Price>`,
    CAST(DATE "2026-12-31" AS DATE) AS `target!: _`,
    JSON_OBJECT(
        'name', club_information.name_full,
        'iban', club_information.bank_iban,
        'bic', club_information.bank_bic
    ) AS `bank: Json<ClubBankInformation>`
FROM registrations
    JOIN events USING (event_id)
    JOIN registration_invoices USING (registration_id)
    JOIN articles USING (article_id)
    CROSS JOIN club_information
WHERE registrations.registration_id = ?
