# pghamming

Postgres extension written in `Rust` using `pgrx` project ! 

## Why ? 
Because nobody seems to be supporting binary vectors. Postgres already has the `bytea` binary type. If you use `phash` embeddings for you data and want to use a vector similiarity search then existing pg vector extension like `pgvector` will be slow. In short, you'll need to represent you data to a vector<float> where each element is {0,1}. Then a hamming distance is basically euclidian distance between these vectors. Sounds slow ? it is ! This is why I decided to reinvent the wheel (a binary wheel this time ) and directly implement vector search on binary data.


## TODO:
- [x] Write and test hamming distance between two &[u8] slices
- [x] Test columnar vs row  tables
- [ ] Implement IVF index using postgres index access API
