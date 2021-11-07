use crate::core::Triple;

pub struct Entailment {
    pub input_length: usize,
    pub output_length: usize,
    pub input_pattern: Box<dyn Fn(&Vec<Triple>) -> bool>,
    pub output_pattern: Box<dyn Fn(&Vec<Triple>) -> Vec<Triple>>
}

impl Entailment {
    pub fn verify(&self, input: &Vec<Triple>) -> bool {
        if input.len() == self.input_length {
            (self.input_pattern)(input)
        } else {
            false
        }
    }
    pub fn apply(&self, input: &Vec<Triple>) -> Vec<Triple> {
        if self.verify(input) {
            let triples = (self.output_pattern)(input);
            if triples.len() == self.output_length {
                triples
            } else {
                dbg!(&input, &triples);
                panic!("Error applying entailment rule: number of output triples: \
                       {} did not match declared output length: {}", triples.len(), self.output_length)
            }
        } else {
            Vec::new()
        }
    }
}

