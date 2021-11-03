# RDF-rs

This crate provides the tools necessary to parse RDF graphs. It currently contains a
full (with very few exceptions) [`Turtle`](http://www.w3.org/TR/turtle/) parser that can parse arbitrary 
URIs, Triples, and Graphs (see [`TurtleParser`](crate::parsing::TurtleParser) for example usage).

# Goals

* To provide a simple and easy-to-use RDF parsing API.
* To act as an inference engine capable of filling a graph with all the triples that can be
inferred from the parsed data.

# Usage

This crate is not on [crates.io](https://crates.io) and thus the `Cargo.toml` entry looks like
the following:

```
[dependencies]
rdf-rs = { git = "https://github.com/CoBrooks/rdf-rs" }
```

[`TurtleParser`]: crate::parsing::TurtleParser

