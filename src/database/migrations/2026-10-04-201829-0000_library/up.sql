-- Adds `created_at` and `updated_at` columns to a table. This should be added to all tables.
CREATE OR REPLACE FUNCTION add_timestamp_columns(_tbl regclass) RETURNS VOID AS $$
BEGIN
	EXECUTE format(
		'ALTER TABLE %s
		 ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
		 ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP',
		_tbl
	);

	PERFORM diesel_manage_updated_at(_tbl);
END;
$$ LANGUAGE plpgsql;

CREATE TYPE media_type AS ENUM ('Video', 'Image');

CREATE TABLE IF NOT EXISTS file_references (
	id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),
	size_bytes INTEGER NOT NULL,
	mime_type TEXT NOT NULL,
	bucket TEXT NOT NULL,
	path TEXT NOT NULL
);

SELECT add_timestamp_columns('file_references');

CREATE TABLE IF NOT EXISTS library_entries (
	id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),
	file_reference_id UUID NOT NULL REFERENCES file_references(id),
	file_type media_type NOT NULL,
	processed_output JSONB NOT NULL
);

SELECT add_timestamp_columns('library_entries');
