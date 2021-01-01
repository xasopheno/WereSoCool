use crate::Defs;
use crate::{ArgMap, FunDef, GenOp, GetLengthRatio, ListOp, NormalForm, Normalize, Op, Substitute};
use num_rational::Rational64;
use weresocool_error::Error;

#[derive(Clone, PartialEq, Debug, Hash)]
pub enum Term {
    Op(Op),
    Nf(NormalForm),
    FunDef(FunDef),
    Lop(ListOp),
    Gen(GenOp),
}

impl Normalize for Term {
    fn apply_to_normal_form(&self, input: &mut NormalForm, defs: &Defs) -> Result<(), Error> {
        match self {
            Term::Op(op) => op.apply_to_normal_form(input, defs),
            Term::Nf(nf) => nf.apply_to_normal_form(input, defs),
            Term::FunDef(_fun) => Err(Error::with_msg("Cannot normalize FunDef.")),
            Term::Lop(lop) => lop.apply_to_normal_form(input, defs),
            Term::Gen(_gen) => Err(Error::with_msg("Cannot normalize Generator.")),
        }
    }
}

impl Substitute for Term {
    fn substitute(
        &self,
        normal_form: &mut NormalForm,
        defs: &Defs,
        arg_map: &ArgMap,
    ) -> Result<Term, Error> {
        match self {
            Term::Op(op) => op.substitute(normal_form, defs, arg_map),
            Term::Nf(nf) => nf.substitute(normal_form, defs, arg_map),
            Term::FunDef(_fun) => Err(Error::with_msg("Cannot call substitute on FunDef.")),
            Term::Lop(lop) => lop.substitute(normal_form, defs, arg_map),
            _ => unimplemented!(),
        }
    }
}

impl GetLengthRatio for Term {
    fn get_length_ratio(&self, defs: &Defs) -> Result<Rational64, Error> {
        match self {
            Term::Op(op) => op.get_length_ratio(defs),
            Term::Nf(nf) => nf.get_length_ratio(defs),
            Term::FunDef(_fun) => Err(Error::with_msg("Cannot get length_ratio of FunDef.")),
            Term::Lop(lop) => lop.get_length_ratio(defs),
            _ => unimplemented!(),
        }
    }
}
