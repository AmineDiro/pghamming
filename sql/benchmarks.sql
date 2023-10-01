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
    2;

--- hamming using l2 
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
    2;