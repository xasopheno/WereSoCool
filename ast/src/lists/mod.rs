pub mod indices;
pub mod normalize_listop;
pub mod substitute_list;

use crate::{GenOp, Term};

#[derive(Clone, PartialEq, Debug, Hash)]
pub enum ListOp {
    Const(Vec<Term>),
    Named(String),
    ListOpIndexed {
        list_op: Box<ListOp>,
        indices: Indices,
    },
    Concat(Vec<ListOp>),
    Gen {
        n: usize,
        gen: Box<GenOp>,
    },
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Indices(pub Vec<Index>);

#[derive(Clone, PartialEq, Debug, Hash)]
pub enum Index {
    /// @ [ i.. ]
    Const { indices: Vec<i64> },
    /// @ [ start:end ]
    Slice {
        start: Option<i64>,
        end: Option<i64>,
        skip: i64,
    },
    /// @ [ Random(n) ]
    Random { n: i64, seed: i64 },
    /// @ [ i | term ]
    IndexAndTerm { index: Box<Index>, term: Term },
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct TermVector {
    term: Term,
    index_terms: Vec<Term>,
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct IndexVector {
    pub index: usize,
    pub index_terms: Vec<Term>,
}
