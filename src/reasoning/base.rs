use crate::reasoning::Entailment;
use crate::core::{ Triple, Graph };

pub trait BaseReasoner {
    fn get_entailment_patterns() -> Vec<Entailment>;
    fn get_inferred_triples(_graph: &Graph) -> Vec<Triple> {
        todo!()
    }
}

