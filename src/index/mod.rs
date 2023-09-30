//! Indexing engine

use pgrx::prelude::*;

mod build;
mod cost_estimate;
mod options;
mod scan;
mod vacuum;

/// Postgres Index AM Router
/// Refer to https://www.postgresql.org/docs/current/index-api.html
#[pg_extern(sql = "
    CREATE OR REPLACE FUNCTION ivf_index_handler(internal) RETURNS index_am_handler
    PARALLEL SAFE IMMUTABLE STRICT COST 0.0001 LANGUAGE c AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
    CREATE ACCESS METHOD ivf TYPE INDEX HANDLER ivf_index_handler;
")]
fn ivf_index_handler(_fc_info: pg_sys::FunctionCallInfo) -> PgBox<pg_sys::IndexAmRoutine> {
    let mut am_routine =
        unsafe { PgBox::<pg_sys::IndexAmRoutine>::alloc_node(pg_sys::NodeTag_T_IndexAmRoutine) };

    am_routine.amstrategies = 1;
    am_routine.amsupport = 0;
    am_routine.amoptsprocnum = 0;
    am_routine.amcanorder = false;
    am_routine.amcanorderbyop = true;
    am_routine.amcanbackward = false;
    am_routine.amcanunique = false;
    am_routine.amcanmulticol = false;
    am_routine.amoptionalkey = true;
    am_routine.amsearcharray = false;
    am_routine.amsearchnulls = false;
    am_routine.amstorage = false;
    am_routine.amclusterable = false;
    am_routine.ampredlocks = false;
    am_routine.amcaninclude = false;
    am_routine.amusemaintenanceworkmem = false;
    am_routine.amkeytype = pgrx::pg_sys::InvalidOid;

    am_routine.amvalidate = Some(am_validate);
    am_routine.ambuild = Some(build::am_build);
    am_routine.ambuildempty = Some(build::am_build_empty);

    am_routine.amvacuumcleanup = Some(vacuum::am_vacuum_cleanup);
    am_routine.amcostestimate = Some(cost_estimate::am_cost_estimate);
    am_routine.amoptions = Some(options::am_options);
    am_routine.ambeginscan = Some(scan::am_begin_scan);
    am_routine.amrescan = Some(scan::am_re_scan);
    am_routine.amgettuple = Some(scan::am_get_tuple);
    am_routine.amgetbitmap = Some(scan::am_get_bitmap);
    am_routine.amendscan = Some(scan::am_end_scan);

    am_routine.into_pg_boxed()
}

#[pg_guard]
pub(crate) extern "C" fn am_validate(_op_class_oid: pg_sys::Oid) -> bool {
    true
}
