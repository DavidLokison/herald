WITH
    map AS (SELECT article_id, policy_birthday FROM article_policy_map_persons WHERE event_id = ?)
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
        WHERE policy_birthday >= birthday
        ORDER BY policy_birthday ASC
        LIMIT 1
    ) m
    INNER JOIN articles a USING (article_id)
ORDER BY ord
