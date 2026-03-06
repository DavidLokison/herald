SELECT HEX(article_id) as id, description, price
FROM articles
WHERE (event_type_slug IS NULL OR event_type_slug = ?)
    AND policy_age IS NULL
    AND policy_flags IS NULL
