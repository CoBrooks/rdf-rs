use std::collections::HashMap;

use crate::core::Triple;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    pub base_prefix: Option<String>,
    pub prefixes: HashMap<String, String>,
    pub triples: Vec<Triple>
}

impl Graph {
    /// Expands all the URIs to have full paths for each resource.
    pub fn apply_metadata(&mut self) {
        let base = self.base_prefix.clone().unwrap_or(String::new());
        let prefixes = self.prefixes.clone();

        self.triples.iter_mut()
            .for_each(|t| {
                t.apply_graph_prefixes(&base, &prefixes);
            });
    }
}

