use crate::reasoning::Entailment;
use crate::core::Triple;

pub trait BaseReasoner {
    fn get_entailment_patterns() -> Vec<Entailment>;
    fn get_inferred_triples(triples: Vec<Triple>, depth: usize) -> Vec<Triple> {
        let rules = Self::get_entailment_patterns();

        let mut new_triples: Vec<Triple> = Vec::new();

        for _ in 0..depth {
            let mut triple_bucket: Vec<Triple> = Vec::new();

            for i in 0..triples.len() {
                for rule in &rules {
                    // Single-input rule pass
                    let triple = &triples[i];
                    if rule.verify(&vec![triple.clone()]) {
                        triple_bucket.append(&mut rule.apply(&vec![triple.clone()]));
                    }

                    // Double-input rule pass
                    for j in i..triples.len() {
                        let triple_2 = &triples[j];
                        if rule.verify(&vec![triple.clone(), triple_2.clone()]) {
                            triple_bucket.append(&mut rule.apply(&vec![triple.clone(), triple_2.clone()]));
                        }
                    }
                }
            }

            new_triples.append(&mut triple_bucket);
        }

        new_triples
    }
}

