use hamming::{distance, distance_fast};
use pgrx::prelude::*;

pgrx::pg_module_magic!();

// Uses fast_hamming if x and y and 8-bytes aligned else falls back to hamming
#[pg_extern]
fn hamming_distance(x: &[u8], y: &[u8]) -> i64 {
    match distance_fast(x, y) {
        Ok(d) => d as i64,
        Err(_) => distance(x, y) as i64,
    }
}

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

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
