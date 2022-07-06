-- Create Markings Table
CREATE TABLE markings(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    definition_type TEXT NOT NULL,
    definition TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz,
    created_by uuid NOT NULL,
    updated_by uuid
);
