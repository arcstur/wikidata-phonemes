CREATE TABLE IF NOT EXISTS language_status
(
    qid TEXT PRIMARY KEY NOT NULL,
    working_user INTEGER,
    is_finished BOOLEAN NOT NULL DEFAULT false
);
