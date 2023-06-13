RESET statement_timeout;
RESET lock_timeout;
RESET idle_in_transaction_session_timeout;
RESET client_encoding;
RESET standard_conforming_strings;
SELECT pg_catalog.set_config('search_path', 'public', true);
RESET check_function_bodies;
RESET xmloption;
RESET client_min_messages;
RESET row_security;

DROP SCHEMA public CASCADE;