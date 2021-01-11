#[macro_use]
extern crate serde;
pub mod ast;
pub mod generator;
pub mod lists;
pub mod operations;
pub mod term;
pub use crate::{
    ast::{Defs, FunDef, Op, Op::*, OscType, ASR},
    generator::{Axis, CoefState, Coefs, GenOp, Generator},
    lists::{normalize_listop::join_list_nf, Index, IndexVector, Indices, ListOp, TermVector},
    operations::{
        helpers::{handle_id_error, join_sequence},
        substitute::substitute_operations,
        ArgMap, GetLengthRatio, NameSet, NormalForm, Normalize, PointOp, Substitute,
    },
    term::Term,
};
