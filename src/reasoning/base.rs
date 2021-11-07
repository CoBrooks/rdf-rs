use crate::reasoning::Entailment;
use crate::core::Triple;

pub trait BaseReasoner {
    fn get_entailment_patterns() -> Vec<Entailment>;
    fn get_inferred_triples(triples: Vec<Triple>, depth: usize) -> Vec<Triple> {
        let rules = Self::get_entailment_patterns();

        let mut new_triples: Vec<Triple> = Vec::new();

        let mut buckets: Vec<Vec<Triple>> = vec![triples.clone()];

        for d in 0..depth {
            let mut triple_bucket = Vec::new();
            let bucket_clone = buckets[d].clone();

            for i in 0..buckets[d].len() {
                for rule in &rules {
                    // Single-input rule pass
                    let triple = &bucket_clone[i];
                    if rule.verify(&vec![triple.clone()]) {
                        triple_bucket.append(&mut rule.apply(&vec![triple.clone()]));
                    }

                    // Double-input rule pass
                    for j in i..buckets[d].len() {
                        let triple_2 = &bucket_clone[j];
                        if rule.verify(&vec![triple.clone(), triple_2.clone()]) {
                            triple_bucket.append(&mut rule.apply(&vec![triple.clone(), triple_2.clone()]));
                        }
                    }
                }
            }

            buckets.push(triple_bucket);
        }

        // The first bucket is the input triples
        let buckets = buckets[1..].to_vec();
        new_triples.append(&mut buckets.into_iter().flatten().collect());

        // Sort and deduplicate
        let triples_as_strings: Vec<String> = triples.into_iter().map(|t| t.to_string()).collect();
        new_triples.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        new_triples.dedup_by(|a, b| a.to_string() == b.to_string());
        new_triples.retain(|t| !triples_as_strings.contains(&t.to_string()));

        new_triples
    }
}

