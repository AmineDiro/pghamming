--- Images Binary query
select
    s.distance,
    s.id
from
    (
        with query as (
            select
                id,
                phash
            from
                images_bin
            limit
                1
        )
        select
            images_bin.id,
            hamming_distance(query.phash, images_bin.phash) as distance
        from
            images_bin,
            query
    ) s
order by
    s.distance
limit
    10;


--- hamming using l2 
EXPLAIN
select
    s.distance,
    s.id
from
    (
        with query as (
            select
                id,
                phash
            from
                images
            limit
                1
        )
        select
            images.id,
            query.phash <-> images.phash as distance
        from
            images,
            query
    ) s
order by
    s.distance
limit
    10;

--- Approximate search using HNSW and IVF
CREATE INDEX phash_ivf_idx ON images USING ivfflat (phash vector_l2_ops) WITH (lists = 1000);
-- OR
CREATE INDEX phash_hnsw_idx ON images USING hnsw (phash vector_l2_ops);
---- 
EXPLAIN ANALYZE
SELECT * FROM images
WHERE id != (SELECT id from images limit 1)
ORDER BY phash<-> (SELECT phash from images limit 1)
LIMIT 10 ;

