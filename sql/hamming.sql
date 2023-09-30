--- 
--- CREATE Extension
---
DROP EXTENSION hamming;
CREATE EXTENSION hamming;

--- 
--- CREATE test table
---
DROP TABLE pages CASCADE;

CREATE TABLE IF NOT EXISTS pages (uuid UUID, hash bytea);

INSERT INTO
    pages (uuid, hash)
SELECT
    gen_random_uuid() AS uuid,
    (
        '\\x' || substring(
            md5(random() :: text || clock_timestamp() :: text),
            1,
            32
        )
    ) :: bytea AS hash
FROM
    generate_series(1, 1000000);

--- 
--- Queries using hamming distance
--- 
select
    s.distance,
    s.uuid
from
    (
        with query as (
            select
                uuid,
                hash
            from
                pages
            limit
                1
        )
        select
            pages.uuid,
            hamming_distance(query.hash, pages.hash) as distance
        from
            pages,
            query
    ) s
where
    s.distance < 50;

--- Full double join
EXPLAIN
select
    p1.uuid,
    p2.uuid,
    hamming_distance(p1.hash,p2.hash) as hammming_distance
from
    pages p1,
    pages p2;

--- 
--- Operator
---- 

with query as (
    select
        uuid,
        hash
    from
        pages
    limit
        1
)
select
    pages.uuid,
    hamming_distance(query.hash, pages.hash) as distance
from
    pages,
    query