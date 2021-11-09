use std::collections::HashMap;
use std::hash::Hash;

use crate::core::{
    Resource,
    Relationship,
    Object,
    uri::UriType
};

#[derive(Clone, Eq, PartialOrd, Ord)]
/// An RDF triple. Typically constructed from a ([`Resource`], [`Relationship`], [`Object`]) tuple with
/// `.into()`.
pub struct Triple {
    pub subject: Resource,
    pub predicate: Relationship,
    pub object: Object
}

impl Triple {
    pub fn matches_pattern(&self, other: &Triple) -> bool {
        let (Resource(s), Relationship(p), o) = self.clone().into();
        let (Resource(pattern_s), Relationship(pattern_p), pattern_o) = other.clone().into();

        if pattern_s.uri_type == UriType::BlankNode || s == pattern_s {
            if pattern_p.uri_type == UriType::BlankNode || p == pattern_p {
                if let Object::Literal(o) = o {
                    if let Object::Literal(pattern_o) = pattern_o {
                        o == pattern_o
                    } else {
                        false
                    }
                } else if let Object::Resource(pattern_o) = pattern_o {
                    if pattern_o.uri_type == UriType::BlankNode {
                        true
                    } else if let Object::Resource(o) = o {
                        o == pattern_o
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(crate) fn apply_graph_prefixes(&mut self, base: &str, prefixes: &HashMap<String, String>) {
        let Resource(subject_uri) = &mut self.subject;
        if subject_uri.uri_type == UriType::PrefixedWithBase || subject_uri.uri_type == UriType::Relative {
            subject_uri.prefix = base.to_string();
            subject_uri.uri_type = UriType::Full;
        } else if subject_uri.uri_type == UriType::Prefixed {
            let pref = format!("{}", subject_uri.prefix);

            if prefixes.contains_key(&pref) {
                subject_uri.prefix = prefixes.get(&pref).unwrap().to_string();
                subject_uri.uri_type = UriType::Full;
            } else if pref.starts_with('_') { // Blank Node
                subject_uri.uri_type = UriType::BlankNode;
            } else {
                panic!("Use of prefix without first defining it: {}", pref)
            }
        }

        let Relationship(predicate_uri) = &mut self.predicate;
        if predicate_uri.uri_type == UriType::PrefixedWithBase || predicate_uri.uri_type == UriType::Relative {
            predicate_uri.prefix = base.to_string();
            predicate_uri.uri_type = UriType::Full;
        } else if predicate_uri.uri_type == UriType::Prefixed {
            let pref = format!("{}", predicate_uri.prefix);

            if prefixes.contains_key(&pref) {
                predicate_uri.prefix = prefixes.get(&pref).unwrap().to_string();
                predicate_uri.uri_type = UriType::Full;
            } else {
                panic!("Use of prefix without first defining it: {}", pref)
            }
        }

        if let Object::Resource(object_uri) = &mut self.object {
            if object_uri.uri_type == UriType::PrefixedWithBase || object_uri.uri_type == UriType::Relative {
                object_uri.prefix = base.to_string();
                object_uri.uri_type = UriType::Full;
            } else if object_uri.uri_type == UriType::Prefixed {
                let pref = format!("{}", object_uri.prefix);

                if prefixes.contains_key(&pref) {
                    object_uri.prefix = prefixes.get(&pref).unwrap().to_string();
                    object_uri.uri_type = UriType::Full;
                } else if pref.starts_with('_') { // Blank Node
                    object_uri.uri_type = UriType::BlankNode;
                } else {
                    panic!("Use of prefix without first defining it: {}", pref)
                }
            }
        }
    }
}

impl From<(Resource, Relationship, Object)> for Triple {
    fn from(triple: (Resource, Relationship, Object)) -> Self {
        let (res, rel, obj) = triple;

        Self {
            subject: res,
            predicate: rel,
            object: obj
        }
    }
}

impl From<Triple> for (Resource, Relationship, Object) {
    fn from(triple: Triple) -> Self {
        (triple.subject, triple.predicate, triple.object)
    }
}

impl ToString for Triple {
    fn to_string(&self) -> String {
        format!("{} {} {} .", self.subject.to_string(), self.predicate.to_string(), self.object.to_string())
    }
}

impl std::fmt::Debug for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Hash for Triple {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

