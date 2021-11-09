use crate::core::{ Object, Triple };

pub struct QueryBuilder {
    triples: Vec<Triple>
}

impl QueryBuilder {
    pub fn start(triples: Vec<Triple>) -> Self {
        Self {
            triples
        }
    }

    pub fn select<F>(mut self, condition: F) -> Self
        where F: Fn(&Triple) -> bool {
        
        self.triples.retain(condition);

        self
    }

    pub fn subject<F>(mut self, condition: F) -> Self 
        where F: Fn(String) -> bool {

        self.triples.retain(|t| condition(t.subject.to_string()));

        self
    }

    pub fn predicate<F>(mut self, condition: F) -> Self 
        where F: Fn(String) -> bool {

        self.triples.retain(|t| condition(t.predicate.to_string()));

        self
    }
    
    pub fn object<F>(mut self, condition: F) -> Self 
        where F: Fn(String) -> bool {

        self.triples.retain(|t| condition(t.object.to_string()));

        self
    }

    pub fn value(self) -> Option<Object> {
        if self.triples.len() > 0 {
            Some(self.triples[0].clone().object)
        } else {
            None
        }
    }

    pub fn values(self) -> Option<Vec<Object>> {
        if self.triples.len() > 0 {
            Some(self.triples.into_iter().map(|t| t.object).collect())
        } else {
            None
        }
    }

    pub fn query(self) -> Vec<Triple> {
        self.triples
    }
}

