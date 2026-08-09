WITH
    params AS (SELECT event_id FROM events WHERE event_id = ?),
    data AS (SELECT * FROM JSON_TABLE(
        ?, "$[*]" COLUMNS(
            ord FOR ORDINALITY,
            birthday DATE PATH "$" ERROR ON ERROR
        )
    ) data),
    map_week AS (SELECT article_id, max_birthday FROM params INNER JOIN event_age_groups USING (event_id) WHERE article_type = "week"),
    map_disc AS (SELECT article_id, max_birthday FROM params INNER JOIN event_age_groups USING (event_id) INNER JOIN event_discounts USING (event_id, article_type)),
    data_week AS (SELECT ord, description, price FROM data INNER JOIN LATERAL (
        SELECT article_id
        FROM map_week
        WHERE max_birthday >= birthday
        ORDER BY max_birthday ASC
        LIMIT 1
    ) m INNER JOIN articles a USING (article_id)),
    data_disc AS (SELECT ord, description, price FROM data INNER JOIN LATERAL (
        SELECT article_id
        FROM map_disc
        WHERE max_birthday >= birthday
        ORDER BY max_birthday ASC
        LIMIT 1
    ) m INNER JOIN articles a USING (article_id))
SELECT CONCAT(data_week.description, COALESCE(CONCAT(', ', data_disc.description), "")) AS description, data_week.price + COALESCE(data_disc.price, 0) AS `price: _`
FROM data_week LEFT JOIN data_disc USING (ord)
ORDER BY ord
