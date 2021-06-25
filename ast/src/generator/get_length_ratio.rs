use crate::generator::{error_non_generator, Axis, Coefs, GenOp, Generator};
use crate::operations::helpers::handle_id_error;
use crate::{GetLengthRatio, NormalForm, Term};
use num_integer::lcm;
use num_rational::Rational64;
use rand::{rngs::StdRng, SeedableRng};
use scop::Defs;
use weresocool_error::Error;

impl GetLengthRatio<Term> for GenOp {
    fn get_length_ratio(
        &self,
        normal_form: &NormalForm,
        defs: &mut Defs<Term>,
    ) -> Result<Rational64, Error> {
        match self {
            GenOp::Named { name, seed } => {
                let generator = handle_id_error(name, defs)?;
                match generator {
                    Term::Gen(mut gen) => {
                        gen.set_seed(*seed);
                        gen.get_length_ratio_genop(None, normal_form, defs)
                    }
                    _ => Err(error_non_generator()),
                }
            }
            GenOp::Const { gen, seed } => {
                let n = gen.lcm_length();
                Ok(gen.get_length(n, *seed, normal_form, defs)?)
            }
            GenOp::Taken { gen, n, seed } => {
                let mut gen = gen.to_owned();
                gen.set_seed(*seed);
                gen.get_length_ratio_genop(Some(*n), normal_form, defs)
            }
        }
    }
}

impl GenOp {
    pub fn length(&self, defs: &mut Defs<Term>) -> Result<usize, Error> {
        match self {
            GenOp::Named { name, seed } => {
                let generator = handle_id_error(name, defs)?;
                match generator {
                    Term::Gen(mut gen) => {
                        gen.set_seed(*seed);
                        gen.length(defs)
                    }
                    _ => Err(error_non_generator()),
                }
            }
            GenOp::Const { gen, .. } => Ok(gen.lcm_length()),
            GenOp::Taken { n, .. } => Ok(*n),
        }
    }

    pub fn get_length_ratio_genop(
        &self,
        n: Option<usize>,
        normal_form: &NormalForm,
        defs: &mut Defs<Term>,
    ) -> Result<Rational64, Error> {
        match self {
            GenOp::Named { name, seed } => {
                let generator = handle_id_error(name, defs)?;
                match generator {
                    Term::Gen(mut gen) => {
                        gen.set_seed(*seed);
                        gen.get_length_ratio_genop(n, normal_form, defs)
                    }
                    _ => Err(error_non_generator()),
                }
            }
            GenOp::Const { gen, seed } => {
                let n = if let Some(n) = n { n } else { gen.lcm_length() };
                Ok(gen.get_length(n, *seed, normal_form, defs)?)
            }
            GenOp::Taken { n, gen, seed } => {
                let mut gen = gen.to_owned();
                gen.set_seed(*seed);
                gen.get_length_ratio_genop(Some(*n), normal_form, defs)
            }
        }
    }
}

impl Generator {
    pub fn get_length(
        &self,
        n: usize,
        seed: u64,
        normal_form: &NormalForm,
        defs: &mut Defs<Term>,
    ) -> Result<Rational64, Error> {
        let mut lengths = vec![Rational64::new(1, 1); n];
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let mut copy = self.clone();

        for length in lengths.iter_mut() {
            for coef in copy.coefs.iter_mut() {
                match coef.axis {
                    Axis::L => {
                        let l = coef
                            .generate(&mut rng)?
                            .get_length_ratio(normal_form, defs)?;
                        *length *= l
                    }
                    _ => {
                        coef.generate(&mut rng)?;
                    }
                }
            }
        }

        let result = Ok(lengths
            .iter()
            .fold(Rational64::from_integer(0), |current, val| current + *val));
        result
    }

    pub fn lcm_length(&self) -> usize {
        let lengths: Vec<usize> = self
            .coefs
            .iter()
            .map(|coef| match &coef.coefs {
                Coefs::Const(c) => c.len(),
                Coefs::Poly(_) => coef.div - 1,
                Coefs::Expr { .. } => coef.div - 1,
            })
            .collect();
        1 + lengths
            .iter()
            .fold(1usize, |current, val| lcm(current, *val))
    }
}
