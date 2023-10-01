# pghamming

Postgres extension written in `Rust` using `pgrx` project !

## Why ?

Because nobody seems to be supporting binary vectors. Postgres already has the `bytea` binary type. If you use `phash` embeddings for you data and want to use a vector similiarity search then existing pg vector extension like `pgvector` will be slow. In short, you'll need to represent you data to a vector<float> where each element is {0,1}. Then a hamming distance is basically euclidian distance between these vectors. Sounds slow ? it is ! This is why I decided to reinvent the wheel (a binary wheel this time ) and directly implement vector search on binary data.

## Benchmark vs pgvector

**Storage**:
Storing binary hashes vs vector. The `python/insert.py` generates random 1_000_000 records representing an image and its perceptual hash and 32-byte binary. The rows are stored in an `images(uuid,vector(256))` and `images_bin(uuid,bytea)`.

| Name       | Size    |
| ---------- | ------- |
| images     | 1116 MB |
| images_bin | 81 MB   |

> 13x storage reduction when using `bytea` format.

**Retrieval**:
Retrieving the 10 nearest neighours to the first using the two tables.

| Name       | Select 10-neighrest neighbors |
| ---------- | ----------------------------- |
| images     | 204 ms                        |
| images_bin | 405 ms                        |

> Thats a 2x speed improvement on a sequential scan

## TODO:

- [x] Write and test hamming distance between two &[u8] slices
- [x] Test columnar vs row tables
- [ ] Implement IVF index using postgres index access API
