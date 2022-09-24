#[macro_use] extern crate lazy_static;
use std::{collections::{HashMap, HashSet}, hash::Hash};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoneOrEmptyParam(String),
    ParseError(String),
    NotAbsoluteURI,
    BaseIsNotAbsolute,
    PrefixIsNotAbsolute,
    NotAValidURI,
    UnknownPrefix(String),
    Unimplemented,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Namespace {
    Absolute(String),
    Prefix(String)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct URI {
    pub namespace: Namespace,
    pub resource: String,
    pub raw: String,
}

type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref URI_ABSOLUTE_REGEX: Regex = Regex::new(r"<((?:http://|https://)\S+[/#])(\S+)>").unwrap();
    static ref URI_PREFIXED_REGEX: Regex = Regex::new(r"(\w+):(\w+)").unwrap();
    static ref URI_NAMESPACE_REGEX: Regex = Regex::new(r"<(?:http://|https://)\S+[/#]>").unwrap();
}

impl URI {
    pub fn new(namespace: Namespace, resource: &str) -> Self {
        let resource = resource.to_string();

        let raw = match namespace {
            Namespace::Absolute(ref ns) => format!("<{ns}{resource}>"),
            Namespace::Prefix(ref prefix) => format!("{prefix}:{resource}"),
        };

        URI {
            namespace,
            resource,
            raw
        }
    }

    pub fn parse(uri: &str) -> Result<URI> {
        if URI_ABSOLUTE_REGEX.is_match(uri) {
            URI::parse_absolute(uri)
        } else if URI_PREFIXED_REGEX.is_match(uri) {
            URI::parse_prefixed(uri)
        } else {
            Err(Error::NotAValidURI)
        }
    }

    pub fn is_absolute(&self) -> bool {
        if let Namespace::Absolute(_) = self.namespace {
            true
        } else {
            false 
        }
    }

    fn parse_absolute(uri: &str) -> Result<URI> {
        let groups = URI_ABSOLUTE_REGEX.captures(uri)
            .map(|c| c.iter().collect::<Vec<_>>());

        if let Some([_, namespace, resource]) = groups.as_deref() {
            let namespace = namespace.unwrap().as_str().to_string();
            let namespace = Namespace::Absolute(namespace);

            let resource = resource.unwrap().as_str().to_string();
            let raw = uri.to_string();

            Ok(URI {
                namespace,
                resource,
                raw
            })
        } else {
            Err(Error::ParseError(format!("Unable to parse absolute URI {uri:?}")))
        }
    }
    
    fn parse_prefixed(uri: &str) -> Result<URI> {
        let groups = URI_PREFIXED_REGEX.captures(uri)
            .map(|c| c.iter().collect::<Vec<_>>());

        if let Some([_, namespace, resource]) = groups.as_deref() {
            let namespace = namespace.unwrap().as_str().to_string();
            let namespace = Namespace::Prefix(namespace);

            let resource = resource.unwrap().as_str().to_string();
            let raw = uri.to_string();

            Ok(URI {
                namespace,
                resource,
                raw
            })
        } else {
            Err(Error::ParseError(format!("Unable to parse absolute URI {uri:?}")))
        }
    }

    fn canonicalize(&self, store: &Store) -> Result<URI> {
        match &self.namespace {
            Namespace::Absolute(_) => Ok(self.clone()),
            Namespace::Prefix(prefix) => {
                if let Some(abs) = store.prefixes.get(prefix) {
                    Ok(
                        URI::new(
                            abs.clone(),
                            &self.resource.clone()
                        )
                    )
                } else {
                    Err(Error::UnknownPrefix(prefix.to_string()))
                }
            }
        }
    }
}

impl ToString for URI {
    fn to_string(&self) -> String {
        let resource = &self.resource;

        match &self.namespace {
            Namespace::Absolute(ns) => format!("<{ns}{resource}>"),
            Namespace::Prefix(prefix) => format!("{prefix}:{resource}")
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Triple {
    pub subject: URI,
    pub predicate: URI,
    pub object: URI
}

impl Triple {
    pub fn new(subject: URI, predicate: URI, object: URI) -> Triple {
        Triple { subject, predicate, object }
    }

    pub fn parse(s: &str) -> Result<Triple> {
        let parts: Vec<&str> = s.split([' ', ';']).collect();

        if let [subject, predicate, object, _] = parts.as_slice() {
            let subject = URI::parse(subject)?;
            let predicate = URI::parse(predicate)?;
            let object = URI::parse(object)?;

            Ok(Triple {
                subject,
                predicate,
                object
            })
        } else {
            Err(Error::ParseError(
                format!("{s:?} is not a triple")
            ))
        }
    }

    pub fn is_absolute(&self) -> bool {
        self.subject.is_absolute() &&
            self.predicate.is_absolute() &&
            self.object.is_absolute()
    }

    fn canonicalize(&self, store: &Store) -> Result<Triple> {
        let subject = self.subject.canonicalize(store)?;
        let predicate = self.predicate.canonicalize(store)?;
        let object = self.object.canonicalize(store)?;

        Ok(Triple { subject, predicate, object })
    }
}

impl ToString for Triple {
    fn to_string(&self) -> String {
        let subject = self.subject.to_string();
        let predicate = self.predicate.to_string();
        let object = self.object.to_string();

        format!("{subject} {predicate} {object} .")
    }
}

impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        if self.is_absolute() {
            self.subject == other.subject &&
                self.predicate == other.predicate &&
                self.object == other.object
        } else {
            panic!("Unable to compare prefixed triple")
        }
    }
}

impl Hash for Triple {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_absolute() {
            self.to_string().hash(state)
        } else {
            panic!("Unable to hash prefixed triple")
        }
    }
}

#[derive(Debug)]
pub struct Store {
    base: Namespace,
    prefixes: HashMap<String, Namespace>,
    triples: HashSet<Triple>
}

impl Store {
    pub fn new(base: Option<Namespace>, prefixes: Option<HashMap<String, Namespace>>) -> Store {
        let base = base.unwrap_or_else(|| 
            Namespace::Absolute("<http://example.com/graph#>".to_string())
        );
        let prefixes = prefixes.unwrap_or_default();

        Store {
            base,
            prefixes,
            triples: HashSet::new()
        }
    }

    pub fn add_prefix(&mut self, prefix: &str, ns: Namespace) {
        self.prefixes.insert(prefix.to_string(), ns);
    }

    pub fn contains(&self, triple: &Triple) -> Result<bool> {
        let triple = triple.canonicalize(self)?;

        let canonical_triples: HashSet<Triple> = self.triples.iter()
            .filter_map(|t| t.canonicalize(self).ok())
            .collect();

        Ok(canonical_triples.contains(&triple))
    }

    pub fn add_triple(&mut self, triple: Triple) -> Result<()> {
        let triple = triple.canonicalize(self)?;

        self.triples.insert(triple);

        Ok(())
    }

    pub fn canonicalize(&mut self) -> Result<()> {
        let triples = self.triples
            .iter()
            .cloned()
            .filter_map(|t| t.canonicalize(self).ok())
            .collect();

        self.triples = triples;

        Ok(())
    }
}

