UPDATE registrations
SET status = (
    SELECT status
    FROM registration_statuses
    WHERE status_slug = ?
)
WHERE registration_id = @RegistrationId
