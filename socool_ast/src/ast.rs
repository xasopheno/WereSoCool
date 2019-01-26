extern crate indexmap;
extern crate num_rational;
use crate::operations::helpers::handle_id_error;
use indexmap::IndexMap;
use num_rational::Rational64;

pub type ParseTable = IndexMap<String, Op>;

trait New<T> {
    fn new() -> T;
}

impl New<ParseTable> for ParseTable {
    fn new() -> ParseTable {
        IndexMap::new()
    }
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub enum Op {
    AsIs,
    Id(Vec<String>),
    //
    Noise,
    Sine,
    Square,
    //
    Reverse,
    FInvert,
    //
    Silence {
        m: Rational64,
    },
    TransposeM {
        m: Rational64,
    },
    TransposeA {
        a: Rational64,
    },
    PanM {
        m: Rational64,
    },
    PanA {
        a: Rational64,
    },
    Gain {
        m: Rational64,
    },
    Length {
        m: Rational64,
    },
    //
    Sequence {
        operations: Vec<Op>,
    },
    Overlay {
        operations: Vec<Op>,
    },
    Compose {
        operations: Vec<Op>,
    },
    Choice {
        operations: Vec<Op>,
    },
    ModulateBy {
        operations: Vec<Op>,
    },
    //
    WithLengthRatioOf {
        with_length_of: Box<Op>,
        main: Box<Op>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Hash)]
pub enum OscType {
    Sine,
    Noise,
    Square,
}

pub fn is_choice_op(op: Op, table: &ParseTable) -> bool {
    match op {
        Op::AsIs {}
        | Op::Sine {}
        | Op::Square {}
        | Op::Noise {}
        | Op::FInvert {}
        | Op::Reverse {}
        | Op::TransposeM { .. }
        | Op::TransposeA { .. }
        | Op::PanA { .. }
        | Op::PanM { .. }
        | Op::Gain { .. }
        | Op::Length { .. }
        | Op::Silence { .. } => false,
        Op::Choice { .. } => true,

        Op::Id(id_vec) => is_choice_op(handle_id_error(id_vec.to_vec(), table), table),
        Op::WithLengthRatioOf { .. } => false,

        Op::Sequence { operations }
        | Op::ModulateBy { operations }
        | Op::Compose { operations }
        | Op::Overlay { operations } => {
            for operation in operations {
                if is_choice_op(operation, table) {
                    return true;
                }
            }
            false
        }
    }
}
