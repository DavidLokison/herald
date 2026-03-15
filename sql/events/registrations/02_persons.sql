INSERT INTO persons
    (registration_id, name, birthday, address, comment, flag_vegetarian, flag_organization)
SELECT @RegistrationId, name, birthday, address, comment, vegetarian, organization
FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            name VARCHAR(128) PATH "$.name",
            birthday DATE PATH "$.birthday",
            address VARCHAR(255) PATH "$.address",
            `comment` TEXT PATH "$.comment",
            vegetarian TINYINT UNSIGNED PATH "$.flags.vegetarian",
            organization TINYINT UNSIGNED PATH "$.flags.team"
        )
    ) data
