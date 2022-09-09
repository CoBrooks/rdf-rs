#[macro_use] extern crate lazy_static;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoneOrEmptyParam(String),
    ParseError(String),
    NotAbsoluteURI,
    BaseIsNotAbsolute,
    PrefixIsNotAbsolute,
    Unimplemented,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum URI {
    Absolute(String),
    Prefixed(String, String)
}

type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    static ref URI_ABSOLUTE_REGEX: Regex = Regex::new(r"<(?:http://|https://)\S+[/#]\S+>").unwrap();
    static ref URI_PREFIXED_REGEX: Regex = Regex::new(r"\w+:\w+").unwrap();
}

impl URI {
    pub fn parse(s: &str) -> Result<URI> {
        URI::parse_absolute(s)
            .or_else(|| URI::parse_prefixed(s))
            .ok_or_else(|| Error::ParseError(format!("{s:?} is not a valid URI")))
    }

    pub fn new_absolute(uri: &str) -> Result<URI> {
        if uri.is_empty() {
            Err(
                Error::NoneOrEmptyParam(stringify!(uri).to_string())
            )
        } else if URI::is_absolute(uri) {
            Ok(URI::Absolute(uri.to_string()))
        } else {
            Err(Error::NotAbsoluteURI)
        }
    }

    pub fn new_prefixed(prefix: &str, resource: &str) -> Result<Self> {
        Ok(
            Self::Prefixed(
                prefix.to_string(),
                resource.to_string()
            )
        )
    }

    fn parse_absolute(input: &str) -> Option<URI> {
        if URI_ABSOLUTE_REGEX.is_match(input) {
            Some(URI::Absolute(input.to_string()))
        } else {
            None
        }
    }

    fn is_absolute(input: &str) -> bool {
        URI_ABSOLUTE_REGEX.is_match(input)
    }
    
    fn parse_prefixed(input: &str) -> Option<URI> {
        if URI_PREFIXED_REGEX.is_match(input) {
            let parts: Vec<&str> = input.split(':').collect();

            if let [prefix, resource] = parts.as_slice() {
                Some(URI::Prefixed(prefix.to_string(), resource.to_string()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Triple {
    pub subject: URI,
    pub predicate: URI,
    pub object: URI
}

impl Triple {
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
}

#[derive(Debug)]
pub struct Store {
    base: URI,
    prefixes: HashMap<String, URI>,
    triples: Vec<Triple>
}

impl Store {
    pub fn new(base: Option<URI>, prefixes: Option<HashMap<String, URI>>) -> Result<Store> {
        let base = base.unwrap_or(URI::parse("<http://example.com/graph#>")?);
        let prefixes = prefixes.unwrap_or_default();

        if let URI::Prefixed(_, _) = base {
            return Err(Error::BaseIsNotAbsolute);
        }

        Ok(Store {
            base,
            prefixes,
            triples: Vec::new()
        })
    }

    pub fn add_prefix(&mut self, prefix: &str, uri: URI) -> Result<()> {
        if let URI::Absolute(_) = uri {
            self.prefixes.insert(prefix.to_string(), uri);

            Ok(())
        } else {
            Err(Error::PrefixIsNotAbsolute)
        }
    }

    pub fn contains(&self, triple: &Triple) -> bool {
        todo!()
    }

    pub fn add_triple(&mut self, triple: Triple) {
        if !self.contains(&triple) {
            self.triples.push(triple);
        }
    }

    pub fn canonicalize_triple(&self, triple: &Triple) -> Result<Triple> {
        Err(Error::Unimplemented)
    }
}
