//! This crate provides the tools necessary to parse RDF graphs. It currently contains a
//! full (with very few exceptions) [`Turtle`](http://www.w3.org/TR/turtle/) parser that can parse arbitrary 
//! URIs, Triples, and Graphs (see [`TurtleParser`](crate::parsing::TurtleParser) for example usage).
//!
//! # Goals
//!
//! * To provide a simple and easy-to-use RDF parsing API.
//! * To act as an inference engine capable of filling a graph with all the triples that can be
//! inferred from the parsed data.
//!
//! # Usage
//!
//! This crate is not on [crates.io](https://crates.io) and thus the `Cargo.toml` entry looks like
//! the following:
//!
//! ```
//! [dependencies]
//! rdf-rs = { git = "https://github.com/CoBrooks/rdf-rs" }
//! ```
//!
//! [`TurtleParser`]: crate::parsing::TurtleParser


/// Contains all of the core rdf data structures, such as [`Uri`](crate::core::Uri) and
/// [`Triple`](crate::core::Triple).
pub mod core {
    pub(crate) mod uri;
    mod resource;
    mod relationship;
    pub(crate) mod object;
    mod triple;
    mod graph;

    pub use uri::Uri;
    pub use resource::Resource;
    pub use relationship::Relationship;
    pub use object::{ Object, Literal };
    pub use triple::Triple;
    pub use graph::Graph;
}

/// Contains the currently-implemented parsers and a base [`RDFParser`](crate::parsing::RDFParser) trait allowing 
/// their creation
pub mod parsing {
    mod base;
    mod turtle;

    pub use base::{ ParserError, Parsed, BaseParser };
    pub use turtle::TurtleParser;

    mod tests;
}

/// Contains the currently-implemented reasoner and a base
/// [`RDFReasoner`](crate::reasoning::RDFReasoner) trait allowing their creation.
pub mod reasoning {
    mod base;
    mod entailment;
    mod rdfs;

    pub use base::BaseReasoner;
    pub use entailment::Entailment;
    pub use rdfs::RDFSReasoner;

    mod tests;
}

