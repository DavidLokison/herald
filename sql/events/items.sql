SELECT HEX(article_id) as id, description, price
FROM article_data_items
    JOIN articles USING (article_id)
WHERE event_type_slug = ?
