WITH
    map AS (SELECT * FROM util_event_article_policy_map WHERE event_id = ?)
SELECT HEX(article_id) AS id, a.description, price
FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            ord FOR ORDINALITY,
            birthday DATE PATH "$.birthday" ERROR ON ERROR,
            team TINYINT UNSIGNED PATH "$.team" DEFAULT "0" ON ERROR
        )
    ) data
    INNER JOIN LATERAL (
        SELECT article_id
        FROM map
        WHERE (policy_flags IS NULL OR policy_flags = team)
            AND (policy_birthday IS NULL OR policy_birthday >= birthday)
        ORDER BY policy_flags DESC, policy_age DESC
        LIMIT 1
    ) m
    INNER JOIN articles a USING (article_id)
ORDER BY ord
