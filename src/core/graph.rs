use std::collections::HashMap;
use std::iter::FromIterator;

use crate::core::{ Resource, Relationship, Object, Triple, Uri, uri::UriType };
use crate::querying::QueryBuilder;
use crate::reasoning::{ RDFSReasoner, BaseReasoner };

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

    pub fn apply_prefixes(&mut self) {
        let base = self.base_prefix.clone().unwrap_or(String::new());

        let prefixes: Vec<String> = self.prefixes.keys().cloned().collect();
        let expanded_prefixes: Vec<String> = self.prefixes.values().cloned().collect();

        // reverse prefix keys/values for later use
        let prefix_lookup: HashMap<String, String> = HashMap::from_iter(expanded_prefixes.iter().cloned().zip(prefixes).into_iter());

        self.triples.iter_mut()
            .for_each(|t| {
                let Resource(subject) = &t.subject;
                if subject.uri_type != UriType::Prefixed {
                    if expanded_prefixes.contains(&subject.prefix) {
                        t.subject = Resource(Uri {
                            prefix: prefix_lookup.get(&subject.prefix).unwrap().to_string(),
                            name: subject.name.clone(),
                            uri_type: UriType::Prefixed
                        })
                    } else if subject.uri_type == UriType::PrefixedWithBase {
                        t.subject = Resource(Uri {
                            prefix: base.clone(),
                            name: subject.name.clone(),
                            uri_type: UriType::Prefixed
                        })
                    }
                }
                
                let Relationship(predicate) = &t.predicate;
                if predicate.uri_type != UriType::Prefixed {
                    if expanded_prefixes.contains(&predicate.prefix) {
                        t.predicate = Relationship(Uri {
                            prefix: prefix_lookup.get(&predicate.prefix).unwrap().to_string(),
                            name: predicate.name.clone(),
                            uri_type: UriType::Prefixed
                        })
                    } else if predicate.uri_type == UriType::PrefixedWithBase {
                        t.predicate = Relationship(Uri {
                            prefix: base.clone(),
                            name: predicate.name.clone(),
                            uri_type: UriType::Prefixed
                        })
                    }
                }

                if t.object.is_resource() {
                    let object = t.object.resource().unwrap();
                    
                    if object.uri_type != UriType::Prefixed {
                        if expanded_prefixes.contains(&object.prefix) {
                            t.object = Object::Resource(Uri {
                                prefix: prefix_lookup.get(&object.prefix).unwrap().to_string(),
                                name: object.name.clone(),
                                uri_type: UriType::Prefixed
                            })
                        } else if object.uri_type == UriType::PrefixedWithBase {
                            t.object = Object::Resource(Uri {
                                prefix: base.clone(),
                                name: object.name.clone(),
                                uri_type: UriType::Prefixed
                            })
                        }
                    }
                }
            });
    }

    pub fn start_query(&self, inferrence_depth: usize) -> QueryBuilder {
        // Normalize triple uri format to UriType::Prefixed
        let mut graph = self.clone();
        graph.apply_prefixes();

        // Add inferred triples
        let mut new_triples = RDFSReasoner::get_inferred_triples(graph.triples.clone(), inferrence_depth);
        graph.triples.append(&mut new_triples);

        QueryBuilder::start(graph.triples)
    }
}

