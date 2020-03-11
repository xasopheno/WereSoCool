use crate::{Index, IndexVector, Indices};
use rand::{rngs::StdRng, Rng, SeedableRng};

impl Indices {
    pub fn get_indices_and_terms(&self, len_list: usize) -> Vec<IndexVector> {
        self.0
            .iter()
            .flat_map(|index| index.get_indices_and_terms(len_list))
            .collect()
    }
}

impl Index {
    pub fn get_indices_and_terms(&self, len_list: usize) -> Vec<IndexVector> {
        match self {
            Index::Const { index } => index
                .iter()
                .map(|i| IndexVector {
                    index: *i as usize,
                    index_terms: vec![],
                })
                .collect(),

            Index::Slice { start, end } => {
                let a = match start {
                    Some(start) => {
                        if *start as usize > len_list as usize {
                            panic!(
                                "Start of slice {} is greater than length of list {}",
                                start, len_list
                            )
                        }
                        *start as usize
                    }
                    None => 0,
                };
                let b = match end {
                    Some(end) => {
                        if (*end as usize) > len_list as usize {
                            panic!(
                                "End of slice {} is greater than length of list {}",
                                end, len_list
                            )
                        }
                        *end as usize
                    }
                    None => (len_list - 1) as usize,
                };

                if a == b {
                    panic! {"start {} and end {} of slice are the same value", a, b};
                };

                let range = if a < b { (a..=b) } else { (b..=a) };

                range
                    .into_iter()
                    .map(|n| IndexVector {
                        index: n as usize,
                        index_terms: vec![],
                    })
                    .collect()
            }
            Index::Random { n, seed } => {
                let mut rng: StdRng = SeedableRng::seed_from_u64(*seed as u64);
                (0..*n)
                    .into_iter()
                    .map(|_| {
                        let n: usize = rng.gen_range(0, len_list);
                        IndexVector {
                            index: n,
                            index_terms: vec![],
                        }
                    })
                    .collect()
            }
            Index::IndexAndTerm { index, term } => {
                let mut result = index.get_indices_and_terms(len_list);
                result
                    .iter_mut()
                    .for_each(|index_vector| index_vector.index_terms.push(term.clone()));

                result
            }
        }
    }
}
