INSERT INTO items
    (registration_id, article_id, comment)
SELECT @RegistrationId, unhex(article_id), comment
FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            article_id CHAR(8) PATH "$.article_id",
            `comment` TEXT PATH "$.comment"
        )
    ) data
