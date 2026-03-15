INSERT INTO registrations
    (event_id, name, address, email, phone, emergency_name, emergency_phone, `comment`)
SELECT ?, name, address, email, phone, emergency_name, emergency_phone, `comment`
FROM JSON_TABLE(
        ?, "$" COLUMNS(
            name VARCHAR(128) PATH "$.name",
            address VARCHAR(255) PATH "$.address",
            email VARCHAR(254) PATH "$.email",
            phone VARCHAR(20) PATH "$.phone",
            emergency_name VARCHAR(128) PATH "$.emergency.name",
            emergency_phone VARCHAR(20) PATH "$.emergency.phone",
            `comment` TEXT PATH "$.comment"
        )
    ) data
