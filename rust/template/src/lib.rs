#![allow(
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables
)]

use std::borrow;
use std::boxed;
use std::ffi;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;
use std::mem;
use std::ops::Deref;
use std::os::raw;
use std::os::unix;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::ptr;
use std::sync;

use differential_dataflow::collection;
use timely::communication;
use timely::dataflow::scopes;
use timely::worker;

use serde::Deserialize;
use serde::Serialize;

use abomonation::Abomonation;
use differential_datalog::arcval;
use differential_datalog::decl_enum_into_record;
use differential_datalog::decl_record_mutator_enum;
use differential_datalog::decl_record_mutator_struct;
use differential_datalog::decl_record_mutator_val_enum;
use differential_datalog::decl_struct_into_record;
use differential_datalog::decl_val_enum_into_record;
use differential_datalog::int::*;
use differential_datalog::program::*;
use differential_datalog::record;
use differential_datalog::record::{FromRecord, IntoRecord, Mutator};
use differential_datalog::uint::*;
use differential_datalog::ConvertRelId;

use fnv::{FnvHashMap, FnvHashSet};
use lazy_static::lazy_static;
use libc::size_t;
use num_traits::identities::One;

pub mod api;
pub mod ovsdb;
pub mod update_handler;

/* FlatBuffers bindings generated by `ddlog` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

/* FlatBuffers code generated by `flatc` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf_generated;

pub fn string_append_str(mut s1: String, s2: &str) -> String {
    s1.push_str(s2);
    s1
}

#[allow(clippy::ptr_arg)]
pub fn string_append(mut s1: String, s2: &String) -> String {
    s1.push_str(s2.as_str());
    s1
}

/// A default implementation of `ConvertRelId` that just forwards calls
/// to generated functions of equal name.
pub struct RelIdConverter {}

impl ConvertRelId for RelIdConverter {
    fn relid2name(relId: RelId) -> Option<&'static str> {
        relid2name(relId)
    }
}

/*- !!!!!!!!!!!!!!!!!!!! -*/
// Don't edit this line
// Code below this point is needed to test-compile template
// code and is not part of the template.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Relations {
    X = 0,
}

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
pub enum Value {
    empty(),
    bool(bool),
}

impl Abomonation for Value {}

impl Default for Value {
    fn default() -> Value {
        Value::bool(false)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        panic!("Value::fmt not implemented")
    }
}

impl IntoRecord for Value {
    fn into_record(self) -> record::Record {
        panic!("Value::into_record not implemented")
    }
}

impl Mutator<Value> for record::Record {
    fn mutate(&self, _x: &mut Value) -> Result<(), String> {
        panic!("Value::mutate not implemented")
    }
}

pub fn relname2id(_rname: &str) -> Option<Relations> {
    panic!("relname2id not implemented")
}

pub fn output_relname_to_id(_rname: &str) -> Option<Relations> {
    panic!("output_relname_to_id not implemented")
}

pub fn input_relname_to_id(_rname: &str) -> Option<Relations> {
    panic!("input_relname_to_id not implemented")
}

pub fn record_update(_file: &mut fs::File, _upd: &record::UpdCmd) {
    panic!("record_update not implemented")
}

pub fn relid2rel(_rid: RelId) -> Option<Relations> {
    panic!("relid2rel not implemented")
}

pub fn relval_from_record(_rel: Relations, _rec: &record::Record) -> Result<Value, String> {
    panic!("relval_from_record not implemented")
}

pub fn relkey_from_record(_rel: Relations, _rec: &record::Record) -> Result<Value, String> {
    panic!("relkey_from_record not implemented")
}

pub fn relid2name(_rid: RelId) -> Option<&'static str> {
    panic!("relid2name not implemented")
}

pub fn relid2cname(_rid: RelId) -> Option<&'static ffi::CStr> {
    panic!("relid2name not implemented")
}

pub fn prog(__update_cb: Box<dyn CBFn<Value>>) -> Program<Value> {
    panic!("prog not implemented")
}

lazy_static! {
    pub static ref RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}

lazy_static! {
    pub static ref INPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}

lazy_static! {
    pub static ref OUTPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = { FnvHashMap::default() };
}
