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
    CREATE OR REPLACE FUNCTION ivf_index_handler(internal) RETURNS index_am_handler PARALLEL SAFE IMMUTABLE STRICT COST 0.0001 LANGUAGE c AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
    CREATE ACCESS METHOD ivf TYPE INDEX HANDLER ivf_index_handler;
")]
fn ivf_index_handler(_fc_info: pg_sys::FunctionCallInfo) -> PgBox<pg_sys::IndexAmRoutine> {
    let mut am_routine =
        unsafe { PgBox::<pg_sys::IndexAmRoutine>::alloc_node(pg_sys::NodeTag_T_IndexAmRoutine) };

    /*
     * Total number of strategies (operators) by which we can traverse/search
     * this AM.  Zero if AM does not have a fixed set of strategy assignments.
     */
    am_routine.amstrategies = 3;
    /* total number of support functions that this AM uses */
    am_routine.amsupport = 0;
    /* opclass options support function number or 0 */
    am_routine.amoptsprocnum = 0;
    /* does AM support ORDER BY indexed column's value? */
    am_routine.amcanorder = false;
    /* does AM support ORDER BY result of an operator on indexed column? */
    /* does AM support backward scanning? */
    /* does AM support UNIQUE indexes? */
    /* does AM support multi-column indexes? */
    /* does AM require scans to have a constraint on the first index column? */
    /* does AM handle ScalarArrayOpExpr quals? */
    /* does AM handle IS NULL/IS NOT NULL quals? */
    /* can index storage data type differ from column data type? */
    /* can an index of this type be clustered on? */
    /* does AM handle predicate locks? */
    /* does AM support parallel scan? */
    /* does AM support columns included with clause INCLUDE? */
    /* does AM use maintenance_work_mem? */
    /* does AM summarize tuples, with at least all tuples in the block
     * summarized in one summary */
    /* OR of parallel vacuum flags */
    /* type of data stored in index, or InvalidOid if variable */

    am_routine.amcanorderbyop = true;
    am_routine.amcanbackward = false;
    am_routine.amcanunique = false;
    am_routine.amcanmulticol = false;
    am_routine.amoptionalkey = false;
    am_routine.amsearcharray = true;
    am_routine.amsearchnulls = false;
    am_routine.amstorage = true;
    am_routine.amclusterable = false;
    am_routine.ampredlocks = true;
    am_routine.amcaninclude = false;
    am_routine.amusemaintenanceworkmem = false;
    am_routine.amkeytype = pgrx::pg_sys::InvalidOid;

    am_routine.amvalidate = Some(am_validate);
    am_routine.ambuild = Some(build::am_build);
    am_routine.ambuildempty = Some(build::am_build_empty);
    am_routine.aminsert = Some(build::am_insert);
    am_routine.ambulkdelete = Some(vacuum::am_bulk_delete);
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
