CREATE TABLE talks (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  title VARCHAR NOT NULL DEFAULT '',
  description TEXT NOT NULL DEFAULT '',
  kind VARCHAR NOT NULL DEFAULT 'meetup',
  happens_on TIMESTAMP NOT NULL DEFAULT now(),
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
)
