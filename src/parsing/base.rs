use std::error::Error;

use crate::core::*;

#[derive(Debug)]
pub struct ParserError(pub String);
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Error> From<T> for ParserError {
    fn from(err: T) -> Self {
        ParserError(err.to_string())
    }
}

pub type Parsed<T> = Result<T, ParserError>;

pub trait BaseParser {
    fn uri(u: &str) -> Parsed<Uri>;
    fn resource(r: &str) -> Parsed<Resource>;
    fn relationship(r: &str) -> Parsed<Relationship>;
    fn object(o: &str) -> Parsed<Object>;
    fn triple(t: &str) -> Parsed<Vec<Triple>>;
    fn graph(g: &str) -> Parsed<Graph>;

    /// Acts as a wrapper around [`RDFParser::graph()`] that automatically reads and 
    /// parses a file.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the file is not a valid Graph.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let triple = TurtleParser::from_file("./test_data/simple.ttl")?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_file(path: &str) -> Parsed<Graph> {
        let file = std::fs::read_to_string(path)?;
        Self::graph(&file)
    }
}
