INSERT INTO persons
    (registration_id, name, birthday, address, comment, food_options)
SELECT @RegistrationId, name, birthday, address, comment, food_options
FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            name VARCHAR(128) PATH "$.name",
            birthday DATE PATH "$.birthday",
            address VARCHAR(255) PATH "$.address",
            `comment` TEXT PATH "$.comment",
            food_options TINYINT UNSIGNED PATH "$.food_options"
        )
    ) data
