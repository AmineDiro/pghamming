use hamming::{distance, distance_fast};
use pgrx::prelude::*;

#[pg_extern]
pub(crate) fn hamming_distance(x: &[u8], y: &[u8]) -> i64 {
    match distance_fast(x, y) {
        Ok(d) => d as i64,
        Err(_) => distance(x, y) as i64,
    }
}

// Uses fast_hamming if x and y and 8-bytes aligned else falls back to hamming
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    // Hammming distance of two exact strings should be 0
    #[pg_test]
    fn test_hamming_distance() {
        let result = Spi::get_one::<i64>("SELECT hamming_distance('test'::bytea,'test'::bytea);");
        assert_eq!(result, Ok(Some(0)));
    }
}
