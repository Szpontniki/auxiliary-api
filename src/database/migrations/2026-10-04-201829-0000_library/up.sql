CREATE TABLE file_references (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	size_bytes INTEGER NOT NULL,
	mime_type TEXT NOT NULL,
	bucket TEXT NOT NULL,
	path TEXT NOT NULL,
);

CREATE TYPE media_type AS ENUM ('Video', 'Image');

CREATE TABLE library_entries (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	file_reference_id UUID NOT NULL REFERENCES file_references(id),
	file_type media_type NOT NULL,
	processed_output JSONB NOT NULL,
);
