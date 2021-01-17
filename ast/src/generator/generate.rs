use crate::{
    generator::error_non_generator, handle_id_error, Axis, CoefState, Coefs, Defs, GenOp,
    Generator, NormalForm, Normalize, Op, Term,
};
use num_rational::Rational64;
use polynomials::Polynomial;
use weresocool_error::Error;
use weresocool_shared::helpers::et_to_rational;

impl CoefState {
    fn generate(&mut self) -> Result<Op, Error> {
        match &self.coefs {
            Coefs::Const(coefs) => {
                let result = self.axis.generate_const(self.state, self.div);
                self.state += coefs[self.idx];
                self.idx += 1;
                self.idx %= coefs.len();
                Ok(result)
            }
            Coefs::Poly(poly) => {
                let result = self.axis.generate_poly(self.state, self.div, poly)?;
                self.state += 1;
                Ok(result)
            }
            _ => unimplemented!(),
        }
    }
}

pub fn eval_polynomial(
    polynomial: &Polynomial<Rational64>,
    state: i64,
    div: usize,
) -> Result<Rational64, Error> {
    let result = polynomial.eval(Rational64::new(state, div as i64));
    if let Some(value) = result {
        Ok(value)
    } else {
        println!("Error: Polynomials must have at least one value.");
        Err(Error::with_msg("Polynomial must have at least one value."))
    }
}

impl Axis {
    pub fn at_least_axis_minimum(&self, r: Rational64, div: usize) -> Rational64 {
        match self {
            Axis::F => std::cmp::max(r, Rational64::from_integer(0)),
            Axis::G => std::cmp::max(r, Rational64::from_integer(0)),
            Axis::L => std::cmp::max(r, Rational64::new(1, div as i64)),
            Axis::P => r,
        }
    }

    fn generate_poly(
        &self,
        state: i64,
        div: usize,
        poly: &Polynomial<Rational64>,
    ) -> Result<Op, Error> {
        let eval = eval_polynomial(poly, state, div)?;

        match self {
            Axis::F => Ok(Op::TransposeM {
                m: self.at_least_axis_minimum(eval, div),
            }),
            Axis::L => Ok(Op::Length {
                m: self.at_least_axis_minimum(eval, div),
            }),
            Axis::G => Ok(Op::Gain {
                m: self.at_least_axis_minimum(eval, div),
            }),
            Axis::P => Ok(Op::PanA {
                a: self.at_least_axis_minimum(eval, div),
            }),
        }
    }
    fn generate_const(&self, state: i64, div: usize) -> Op {
        match self {
            Axis::F => Op::TransposeM {
                m: self.at_least_axis_minimum(et_to_rational(state, div), div),
            },
            Axis::L => Op::Length {
                m: self.at_least_axis_minimum(Rational64::new(state, div as i64), div),
            },
            Axis::G => Op::Gain {
                m: self.at_least_axis_minimum(Rational64::new(state, div as i64), div),
            },
            Axis::P => Op::PanA {
                a: Rational64::new(state, div as i64),
            },
        }
    }
}

impl Generator {
    pub fn generate(
        &mut self,
        nf: &NormalForm,
        n: usize,
        defs: &Defs,
    ) -> Result<Vec<NormalForm>, Error> {
        let mut result: Vec<NormalForm> = vec![];
        let mut coefs = self.coefs.clone();

        for _ in 0..n {
            let mut nf: NormalForm = nf.clone();
            for coef in coefs.iter_mut() {
                coef.generate()?.apply_to_normal_form(&mut nf, defs)?;
            }
            result.push(nf)
        }

        Ok(result)
    }
}

impl GenOp {
    pub fn generate_from_genop(
        self,
        input: &mut NormalForm,
        n: Option<usize>,
        defs: &Defs,
    ) -> Result<Vec<NormalForm>, Error> {
        match self {
            GenOp::Named(name) => {
                let generator = handle_id_error(name, defs, None)?;
                match generator {
                    Term::Gen(gen) => gen.generate_from_genop(input, n, defs),
                    _ => Err(error_non_generator()),
                }
            }
            GenOp::Const(mut gen) => {
                let length = if let Some(n) = n { n } else { gen.lcm_length() };
                gen.generate(input, length, defs)
            }

            GenOp::Taken { gen, n } => gen.generate_from_genop(input, Some(n), defs),
        }
    }
}
