use pgrx::{prelude::*, FromDatum, PgRelation};

struct IVFIndexOptions {
    nlists: usize,
}
struct IVFBuider {
    nlists: usize,
    ntuples: f32,
}

impl IVFBuider {
    fn new() -> Self {
        Self {
            nlists: 10,
            ntuples: 0.0,
        }
    }
}

#[cfg(any(feature = "pg11", feature = "pg12"))]
#[pg_guard]
unsafe extern "C" fn callback(
    _index_relation: pg_sys::Relation,
    htup: pg_sys::HeapTuple,
    values: *mut pg_sys::Datum,
    is_null: *mut bool,
    _tuple_is_alive: bool,
    state: *mut std::os::raw::c_void,
) {
    unimplemented!()
}

#[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
#[pgrx::pg_guard]
unsafe extern "C" fn build_callback(
    _index_relation: pgrx::pg_sys::Relation,
    ctid: pgrx::pg_sys::ItemPointer,
    values: *mut pgrx::pg_sys::Datum,
    is_null: *mut bool,
    _tuple_is_alive: bool,
    state: *mut std::os::raw::c_void,
) {
    let builder = &mut *(state as *mut IVFBuider);
    let val: Option<&[u8]> = FromDatum::from_datum(*values, *is_null);
    if let Some(hash) = val {
        info!("{}: {:?}", builder.ntuples, hash.len());
        // TODO : Index this !
    }
    builder.ntuples += 1.0;
}

#[pg_guard]
pub(crate) unsafe extern "C" fn am_build(
    heap_relation: pg_sys::Relation,
    index_relation: pg_sys::Relation,
    index_info: *mut pg_sys::IndexInfo,
) -> *mut pg_sys::IndexBuildResult {
    // Convert to rust wrapper types
    let heap = { PgRelation::from_pg(heap_relation) };
    let index = unsafe { PgRelation::from_pg(index_relation) };
    let mut builder = IVFBuider::new();
    unsafe {
        pg_sys::IndexBuildHeapScan(
            heap.as_ptr(),
            index.as_ptr(),
            index_info,
            Some(build_callback),
            &mut builder,
        );
    }

    // Construct result
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
