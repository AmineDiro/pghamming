-- After adding adding shared_preload_libraries = 'citus'
-- to postgresql.conf and restarting:
CREATE EXTENSION IF NOT EXISTS citus;

-- Create a table using the columnar access method, with the same columns
-- as an existing cstore_fdw table
CREATE TABLE pages_am (
  LIKE  pages INCLUDING ALL
) USING columnar;

-- Copy data from an old cstore_fdw table to an access method table
INSERT INTO pages_am SELECT * FROM pages;

-- cstore_fdw data size
SELECT pg_size_pretty(cstore_table_size('pages_am'));

-- Citus Columnar data size
SELECT pg_size_pretty(pg_table_size('pages'));