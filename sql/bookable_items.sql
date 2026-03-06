SELECT article_id as id, description, price
FROM api_item_articles
WHERE event_type_slug IS NULL OR event_type_slug = ?
