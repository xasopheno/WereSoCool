use crate::operations::{helpers::handle_id_error, ArgMap, NormalForm, Normalize, Substitute};
use crate::substitute_operations;
use crate::{Defs, ListOp, Term};
use weresocool_error::Error;

impl Substitute for ListOp {
    fn substitute(
        &self,
        normal_form: &mut NormalForm,
        defs: &Defs,
        arg_map: &ArgMap,
    ) -> Result<Term, Error> {
        match self {
            ListOp::Const(terms) => Ok(Term::Lop(ListOp::Const(substitute_operations(
                terms.to_vec(),
                normal_form,
                defs,
                arg_map,
            )?))),
            ListOp::Named(name) => {
                let term = handle_id_error(name.to_string(), defs, Some(arg_map));

                match term {
                    Term::Lop(lop) => Ok(lop.substitute(normal_form, defs, arg_map)?),
                    _ => unimplemented!(),
                }
            }
            ListOp::ListOpIndexed { .. } => Ok(Term::Lop(ListOp::Const(
                self.term_vectors(defs, Some(arg_map))
                    .iter_mut()
                    .map(|term_vector| {
                        let mut nf = normal_form.clone();

                        term_vector.term.apply_to_normal_form(&mut nf, defs);
                        term_vector
                            .index_terms
                            .iter()
                            .map(|index_term| index_term.apply_to_normal_form(&mut nf, defs))
                            .collect::<Result<Vec<_>, Error>>();

                        Term::Nf(nf)
                    })
                    .collect(),
            ))),
            ListOp::Concat(lists) => {
                let mut result = vec![];
                for list in lists {
                    result.push(list.substitute(normal_form, defs, arg_map)?)
                }

                Ok(Term::Lop(ListOp::Const(result)))
            }
        }
    }
}
