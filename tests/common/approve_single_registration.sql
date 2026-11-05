SET @RegistrationId = (SELECT registration_id FROM registrations);
CALL approve_registration(@RegistrationId);
