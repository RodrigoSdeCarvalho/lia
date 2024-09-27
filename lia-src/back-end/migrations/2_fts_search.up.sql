ALTER TABLE commands
ADD COLUMN search_vector tsvector;

UPDATE commands
SET search_vector = to_tsvector('english', coalesce(name, '') || ' ' || coalesce(description, '') || ' ' || coalesce(command_text, ''));

CREATE FUNCTION update_search_vector() RETURNS trigger AS $$
BEGIN
  NEW.search_vector := to_tsvector('english', coalesce(NEW.name, '') || ' ' || coalesce(NEW.description, '') || ' ' || coalesce(NEW.command_text, ''));
  RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER commands_search_vector_update
BEFORE INSERT OR UPDATE ON commands
FOR EACH ROW EXECUTE FUNCTION update_search_vector();

CREATE INDEX idx_commands_search_vector ON commands USING GIN(search_vector);
