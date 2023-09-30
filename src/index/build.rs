use pgrx::prelude::*;

#[pg_guard]
pub(crate) unsafe extern "C" fn am_build(
    heap_relation: pg_sys::Relation,
    index_relation: pg_sys::Relation,
    index_info: *mut pg_sys::IndexInfo,
) -> *mut pg_sys::IndexBuildResult {
    // index_build::build(index_relation, Some((heap_relation, index_info)));
    let mut result = pgrx::PgBox::<pgrx::pg_sys::IndexBuildResult>::alloc0();
    result.heap_tuples = 0.0;
    result.index_tuples = 0.0;
    result.into_pg()
}

#[pg_guard]
pub(crate) extern "C" fn am_build_empty(index_relation: pg_sys::Relation) {
    unimplemented!()
}

#[cfg(any(feature = "pg10", feature = "pg11", feature = "pg12", feature = "pg13"))]
#[pg_guard]
pub(crate) extern "C" fn am_insert(
    index_relation: pg_sys::Relation,
    values: *mut pg_sys::Datum,
    is_null: *mut bool,
    heap_tid: pg_sys::ItemPointer,
    heap_relation: pg_sys::Relation,
    check_unique: pg_sys::IndexUniqueCheck,
    index_info: *mut pg_sys::IndexInfo,
) -> bool {
    am_insert_internal()
}

#[cfg(any(feature = "pg14", feature = "pg15", feature = "pg16"))]
#[pg_guard]
pub(crate) extern "C" fn am_insert(
    index_relation: pg_sys::Relation,
    values: *mut pg_sys::Datum,
    is_null: *mut bool,
    heap_tid: pg_sys::ItemPointer,
    heap_relation: pg_sys::Relation,
    check_unique: pg_sys::IndexUniqueCheck,
    index_unchanged: bool,
    index_info: *mut pg_sys::IndexInfo,
) -> bool {
    am_insert_internal()
}

#[inline(always)]
fn am_insert_internal() -> bool {
    unimplemented!()
}
