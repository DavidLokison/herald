UPDATE registrations
SET status = (
    SELECT status
    FROM registration_statuses
    WHERE status_slug = "awaiting_approval"
)
WHERE registration_id = @RegistrationId
