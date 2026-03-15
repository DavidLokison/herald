INSERT INTO items
    (registration_id, article_id, comment)
SELECT @RegistrationId, article_id, comment
FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            article_id BINARY(4) PATH "$.article_id",
            `comment` TEXT PATH "$.comment"
        )
    ) data
