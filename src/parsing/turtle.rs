use std::collections::HashMap;

use if_chain::if_chain;

use crate::core::*;
use crate::parsing::base::{
    Parsed,
    BaseParser,
    ParserError
};

#[derive(Clone, PartialEq)]
enum Keyword {
    Prefix,
    Base
}

impl std::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Keyword::Base => write!(f, "@base"),
            Keyword::Prefix => write!(f, "@prefix"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Token {
    Keyword(Keyword),
    Whitespace,
    TripleSep,
    PredicateSep,
    ObjectSep,
    Word(String),
    PropertyListOpen,
    PropertyListClose,
    CollectionOpen,
    CollectionClose,
}

impl Token {
    pub fn vec_to_string(tokens: Vec<Token>) -> String {
        let mut s = String::new();
        
        for t in tokens.iter().filter(|&t| t != &Token::Whitespace) {
            s += &format!("{:?} ", t);
        }

        s
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::Keyword(kw) => write!(f, "{:?}", kw),
            Token::Whitespace => write!(f, " "),
            Token::TripleSep => write!(f, "."),
            Token::PredicateSep => write!(f, ";"),
            Token::ObjectSep => write!(f, ","),
            Token::Word(word) => write!(f, "{}", word),
            Token::PropertyListOpen => write!(f, "["),
            Token::PropertyListClose => write!(f, "]"),
            Token::CollectionOpen => write!(f, "("),
            Token::CollectionClose => write!(f, ")"),
        }
    }
}

pub struct TurtleParser;
impl TurtleParser {
    // A very simple lexer to tokenize rdf input for later parsing
    fn tokenize(s: &str) -> Vec<Token> {
        let s = s.replace("\r\n", "\n");

        let chars: Vec<char> = s.chars().collect();

        let mut tokens: Vec<Token> = Vec::new();
        let mut current: String = String::new();
        
        let mut quoted = false;

        let mut sequential_quotes = 0;
        let mut block_quoted = false;

        let mut commented = false;

        for c in chars {
            let ignored = quoted || block_quoted || commented;

            match c {
                c if (c.is_whitespace() || c == ',') && !ignored => {
                    if !current.is_empty() {
                        match &current as &str {
                            "@prefix" | "PREFIX" => {
                                tokens.push(Token::Keyword(Keyword::Prefix));
                            },
                            "@base" | "BASE" => {
                                tokens.push(Token::Keyword(Keyword::Base));
                            },
                            _ => { 
                                tokens.push(Token::Word(current));
                            }
                        }

                        current = String::new();
                    }
                    
                    if c == ',' {
                        tokens.push(Token::ObjectSep);
                    } else {
                        tokens.push(Token::Whitespace);
                    }

                    sequential_quotes = 0;
                },
                '.' if current.is_empty() && !ignored => {
                    tokens.push(Token::TripleSep);

                    sequential_quotes = 0;
                },
                ';' if current.is_empty() && !ignored => {
                    tokens.push(Token::PredicateSep);

                    sequential_quotes = 0;
                },
                '[' if current.is_empty() && !ignored => {
                    tokens.push(Token::PropertyListOpen);

                    sequential_quotes = 0;
                },
                ']' if current.is_empty() && !ignored => {
                    tokens.push(Token::PropertyListClose);

                    sequential_quotes = 0;
                },
                '(' if current.is_empty() && !ignored => {
                    tokens.push(Token::CollectionOpen);

                    sequential_quotes = 0;
                },
                ')' if current.is_empty() && !ignored => {
                    tokens.push(Token::CollectionClose);

                    sequential_quotes = 0;
                },
                '#' if current.is_empty() && !ignored => {
                    commented = true;
                },
                '\n' if ignored => {
                    commented = false;
                }
                _ if !commented => { 
                    if c == '"' { 
                        quoted = !quoted;

                        sequential_quotes += 1;
                        if sequential_quotes == 3 {
                            block_quoted = !block_quoted;
                            sequential_quotes = 0;
                            quoted = false;
                        }
                    }
                    current.push(c);
                },
                _ => { }
            }
        }

        tokens
    }

    // Expands an rdf collection into its corresponding blank property list format
    // (https://w3.org/TR/turtle/ Examples 20 and 21)
    fn expand_collection_tokens_naive(mut tokens: Vec<Token>) -> Parsed<Vec<Token>> {
        let mut expanded: Vec<Token> = vec![Token::PropertyListOpen];

        let last: Token = tokens.pop().unwrap();
        
        // Will fail if the collection contains anything but Token::Word(..)s 
        // TODO: more thorough collection unwrapping
        for word in &tokens {
            if let Token::Word(word) = word {
                expanded.append(&mut vec![
                    Token::Word("rdf:first".into()),
                    Token::Word(word.to_string()),
                    Token::PredicateSep,
                    Token::Word("rdf:rest".into()),
                    Token::PropertyListOpen
                ])
            } else {
                return Err(ParserError(format!("Collections can only contain Words; {:?}", word)));
            }
        }
        
        expanded.append(&mut vec![
            Token::Word("rdf:first".into()),
            last,
            Token::PredicateSep,
            Token::Word("rdf:rest".into()),
            Token::Word("rdf:nil".into()),
            Token::PropertyListClose,
            Token::PropertyListClose
        ]);

        Ok(expanded)
    }
    
    fn parse_triple_recursive(mut tokens: Vec<Token>, mut triples: Vec<Triple>, blank_node_num: &mut usize) -> Parsed<Vec<Triple>> {
        // Expand collections into blank property lists.
        if tokens.contains(&Token::CollectionOpen) {
            // Get the index of the first open paren
            let first_open_index = tokens.iter().position(|t| t == &Token::CollectionOpen).unwrap();

            // Get the index of the associated closing paren
            let mut depth = 0;
            let mut opened = false;
            let collection_close_index = tokens.iter().position(|t| {
                if let Token::CollectionOpen = t {
                    opened = true;
                    depth += 1;
                } else if let Token::CollectionClose = t {
                    depth -= 1;
                }

                depth == 0 && opened
            }).unwrap();

            // Get the tokens just within the parens
            let collection_tokens = tokens[first_open_index + 1..collection_close_index].to_vec();
            // and expand them into nested blank property lists.
            let collection_tokens = Self::expand_collection_tokens_naive(collection_tokens)?;

            // Replace the collection in the original token list with the expanded version
            tokens.splice(first_open_index..=collection_close_index, collection_tokens);
            // Parse the new expanded version of the tokens.
            return Ok(Self::parse_triple_recursive(tokens, triples, blank_node_num)?);
        }

        // If the first token is a word (the subject)...
        if let Token::Word(subject) = &tokens[0] {
            // ...And the second token is a word (the predicate)...
            if let Token::Word(predicate) = &tokens[1] {
                // ...And the third token is a word (the object)...
                if let Token::Word(object) = &tokens[2] {
                    // Then add this triple to the list.
                    triples.push(
                       (Self::resource(&subject)?, Self::relationship(&predicate)?, Self::object(&object)?).into()
                    );

                    // If this is the end of the triple,
                    if let Token::TripleSep = &tokens[3] {
                        // return the triples.
                        Ok(triples)
                    // If the triple continues with a list of predicates,
                    } else if let Token::PredicateSep = &tokens[3] {
                        // Remove the object and predicate pair that was just parsed
                        tokens.drain(1..=3);
                        // and continue parsing.
                        Ok(Self::parse_triple_recursive(tokens, triples, blank_node_num)?)
                    // If the triple continues with a list of objects,
                    } else if let Token::ObjectSep = &tokens[3] {
                        // Remove the object that was just parsed
                        tokens.drain(2..=3);
                        // and continue parsing
                        Ok(Self::parse_triple_recursive(tokens, triples, blank_node_num)?)
                    } else {
                        Err(ParserError(format!("Triple must end with ' .' or continue with ',' or ' ;'. Found: {:?}", &tokens[3])))
                    }
                // ...And the object is a blank property list...
                } else if let Token::PropertyListOpen = &tokens[2] {
                    // First, increment the node_num for variable naming
                    *blank_node_num += 1;

                    // Second, get just the inner portion of the list
                    let object_tokens = &tokens[2..].to_vec();
                    let mut depth: i8 = 0;
                    let mut i = 0;
                    let token_parts = object_tokens.splitn(2, |t| {
                        i += 1;
                        if let Token::PropertyListOpen = t {
                            depth += 1;
                        } else if let Token::PropertyListClose = t {
                            depth -= 1;
                        }

                        depth == 0 && i > 2
                    }).collect::<Vec<&[Token]>>();

                    let mut inner = token_parts[0].to_vec();

                    // Replace the opening brace with a blank subject token
                    let object = format!("_:blank{}", blank_node_num);
                    inner[0] = Token::Word(object.clone());
                    // And append with a triple terminator
                    inner.push(Token::TripleSep);

                    // Get the list of triples from within the blank prop list
                    let inner_triples = Self::parse_triple_recursive(inner, triples.clone(), blank_node_num)?;

                    // Insert the subject of the prop list as the object of the current triple
                    let mut tokens = tokens[..2].to_vec();
                    tokens.push(Token::Word(object));
                    tokens.append(&mut token_parts[1].to_vec());
                    
                    // Rerun with new tokens and triples
                    Ok(Self::parse_triple_recursive(tokens, inner_triples, blank_node_num)?)
                } else {
                    Err(ParserError(format!("Object must be a resource, literal, or a property list. Found: {:?}", &tokens[2])))
                }
            } else {
                Err(ParserError(format!("Predicate must be a valid URI. Found: {:?}", &tokens[1])))
            }
        // If the subject is a blank property list...
        } else if let Token::PropertyListOpen = &tokens[0] {
            // First, increment the node_num for variable naming
            *blank_node_num += 1;

            // Second, get just the inner portion of the list
            let mut depth: i8 = 0;
            let token_parts = tokens.splitn(2, |t| {
                if let Token::PropertyListOpen = t {
                    depth += 1;
                } else if let Token::PropertyListClose = t {
                    depth -= 1;
                }

                depth == 0
            }).collect::<Vec<&[Token]>>();

            let mut inner = token_parts[0].to_vec();

            // Replace the opening brace with a blank subject token
            inner[0] = Token::Word(format!("_:blank{}", blank_node_num));
            // and append with a Triple terminator
            inner.push(Token::TripleSep);

            // Get the list of triples from the inner section of the blank prop list
            let inner_triples = Self::parse_triple_recursive(inner, triples.clone(), blank_node_num)?;
            
            // Insert the subject of the prop list as the subject of the current triple
            let mut tokens = token_parts[1].to_vec();
            tokens.insert(0, Token::Word(inner_triples[0].subject.to_string()));

            // Rerun with new triples and tokens
            Ok(Self::parse_triple_recursive(tokens, inner_triples, blank_node_num)?)
        } else {
            Err(ParserError(format!("Subject must be a valid URI or a blank property list. Found: {:?}", &tokens[0])))
        }
    }
}

impl BaseParser for TurtleParser {
    /// Parses a [`Uri`] from a string
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid URI
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let uri = TurtleParser::uri("<http://example.com/rdf/Person>")?;
    /// let uri = TurtleParser::uri("<#Person>")?;
    /// let uri = TurtleParser::uri("ex:Person")?;
    /// let uri = TurtleParser::uri(":Person")?;
    /// # Ok(())
    /// # }
    /// ```
    fn uri(u: &str) -> Parsed<Uri> {
        use crate::core::uri::{ UriType, matches };

        let full_url = &matches::FULL_URL;
        let relative_url = &matches::RELATIVE_URL;
        let prefixed = &matches::PREFIXED;
        let empty_prefix = &matches::EMPTY_PREFIX;

        // Trim the leading and trailing whitespace.
        let u = u.trim();

        // If u is in the form <http://[valid url]/foo> or <http://[valid url]#foo>
        let (prefix, name, uri_type) = if full_url.is_match(u) {
            let caps = full_url.captures(u).unwrap();

            (caps[1].to_string(), caps[2].to_string(), UriType::Full)
        // If u is in the form <#foo>
        } else if relative_url.is_match(u) {
            let caps = relative_url.captures(u).unwrap();

            ("".into(), caps[1].to_string(), UriType::Relative)
        // If u is in the form prefix:foo
        } else if prefixed.is_match(u) {
            let caps = prefixed.captures(u).unwrap();

            (caps[1].to_string(), caps[2].to_string(), UriType::Prefixed)
        // If u is in the form :foo
        } else if empty_prefix.is_match(u) {
            let caps = empty_prefix.captures(u).unwrap();

            ("".into(), caps[1].to_string(), UriType::PrefixedWithBase)
        // If u is the identity relationship
        } else if u == "a" {
            ("http://www.w3.org/1999/02/22-rdf-syntax-ns/".into(), "type".into(), UriType::Full)
        } else {
            return Err(ParserError(format!("Invalid URI: {}", u)));
        };

        Ok(Uri::new(&prefix, &name, uri_type))
    }

    /// Parses a [`Resource`] from a string. A wrapper around [`RDFParser::uri()`] specifically 
    /// for RDF resources.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid Resource
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let res = TurtleParser::resource("<http://example.com/rdf/Person>")?;
    /// let res = TurtleParser::resource("<#Person>")?;
    /// let res = TurtleParser::resource("ex:Person")?;
    /// let res = TurtleParser::resource(":Person")?;
    /// # Ok(())
    /// # }
    /// ```
    fn resource(r: &str) -> Parsed<Resource> {
        let uri = Self::uri(r)?;
        Ok(Resource(uri))
    }

    /// Parses a [`Relationship`] from a string. A wrapper around [`RDFParser::uri()`] specifically 
    /// for RDF relationships.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid Relationship
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let rel = TurtleParser::relationship("<http://example.com/foaf#knows>")?;
    /// let rel = TurtleParser::relationship("<#knows>")?;
    /// let rel = TurtleParser::relationship("foaf:knows")?;
    /// let rel = TurtleParser::relationship(":knows")?;
    /// # Ok(())
    /// # }
    /// ```
    fn relationship(r: &str) -> Parsed<Relationship> {
        let uri = Self::uri(r)?;
        Ok(Relationship(uri))
    }

    /// Parses an [`Object`] from a string.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid Object
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let obj = TurtleParser::object(r#""john@example.com""#)?;
    /// let obj = TurtleParser::object(r#""すし"@jp"#)?;
    /// let obj = TurtleParser::object("foaf:Person")?;
    /// # Ok(())
    /// # }
    /// ```
    fn object(o: &str) -> Parsed<Object> {
        use crate::core::object::matches;
        let with_datatype = &matches::WITH_DATATYPE;
        let with_lang = &matches::WITH_LANG;

        // If o is in the form "literal"^^datatype:uri
        if with_datatype.is_match(o) {
            let caps = with_datatype.captures(o).unwrap();
            Ok(Object::Literal(Literal{
                value: caps[1].to_string(),
                datatype: Self::uri(&caps[2])?,
                language: None
            }))
        // If o is in the form "some string"@lang
        } else if with_lang.is_match(o) {
            let caps = with_lang.captures(o).unwrap();
            Ok(Object::Literal(Literal{
                value: caps[1].to_string(),
                datatype: Self::uri("xsd:string")?,
                language: Some(caps[2].to_string())
            }))
        // If o is a valid URI
        } else if let Ok(uri) = Self::uri(o) {
            Ok(Object::Resource(uri))
        // If o is a boolean
        } else if o == "true" || o == "false" {
            Ok(Object::Literal(Literal{
                value: o.to_string(),
                datatype: Self::uri("xsd:boolean")?,
                language: None
            }))
        // Else, o is a string literal
        } else {
            Ok(Object::Literal(Literal {
                value: o.to_string(),
                datatype: Self::uri("xsd:string")?,
                language: None
            }))
        }
    }

    /// Parses a [`Vec<Triple>`] from a string.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid Triple or 
    /// collection of triples.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let triple = TurtleParser::triple(r#"ex:John foaf:mbox "john@example.com" ."#)?;
    /// let triple = TurtleParser::triple(r#"[ foaf:name "Alice" ] foaf:knows [ foaf:name "Bob" ] ."#)?;
    /// # Ok(())
    /// # }
    /// ```
    fn triple(t: &str) -> Parsed<Vec<Triple>> {
        // Strip all of the whitespace tokens
        let tokens: Vec<Token> = Self::tokenize(t).into_iter().filter(|t| t != &Token::Whitespace).collect();

        Ok(Self::parse_triple_recursive(tokens, Vec::new(), &mut 0)?)
    }
    
    /// Parses a [`Graph`] from a string (typically a file).
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the string is not a valid Graph.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rdf_rs::parsing::{ TurtleParser, RDFParser, ParserError };
    /// # fn main() -> Result<(), ParserError> {
    /// let triple = TurtleParser::graph(r#"
    ///     @base <http://example.com/> .
    ///     @prefix foaf: <http://xmlns.com/foaf/0.1/> .
    ///     @prefix owl: <http://www.w3.org/2002/07/owl#> .
    ///
    ///     :John foaf:mbox "john@example.com" .
    ///     [ foaf:mbox "john@example.com" ] owl:sameAs :John .
    /// "#)?;
    /// # Ok(())
    /// # }
    /// ```
    fn graph(g: &str) -> Parsed<Graph> {
        let tokens: Vec<Token> = Self::tokenize(g);
        let mut base_prefix: Option<String> = None;

        // Set the base of the graph if it exists
        if tokens.contains(&Token::Keyword(Keyword::Base)) {
            tokens.iter()
                .filter(|&t| t != &Token::Whitespace)
                .collect::<Vec<&Token>>()
                .split_inclusive(|&t| t == &Token::TripleSep)
                .for_each(|w| {
                    if_chain! {
                        if let Token::Keyword(Keyword::Base) = &w[0];
                        if let Token::Word(prefix) = &w[1];
                        if let Token::TripleSep = &w[2];
                        then {
                            base_prefix = Some(prefix.replace(|c| { "<>".contains(c) }, ""));
                        }
                    }
                });
        }

        let mut prefixes: HashMap<String, String> = HashMap::new();

        // default prefixes
        prefixes.insert("rdf:".into(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into());
        prefixes.insert("xsd:".into(), "http://www.w3.org/2001/XMLSchema#".into());

        // If the graph contains prefixes, parse them
        if tokens.contains(&Token::Keyword(Keyword::Prefix)) {
            tokens.iter()
                .filter(|&t| t != &Token::Whitespace)
                .collect::<Vec<&Token>>()
                .split_inclusive(|&t| t == &Token::TripleSep)
                .for_each(|w| {
                    if_chain! {
                        if let Token::Keyword(Keyword::Prefix) = &w[0];
                        if let Token::Word(prefix) = &w[1];
                        if let Token::Word(expanded) = &w[2];
                        if let Token::TripleSep = &w[3];
                        then {
                            prefixes.insert(prefix.to_string(), expanded.replace(|c| { "<>".contains(c) }, ""));
                        }
                    }
                });
        }

        let full_triples: Vec<String> = tokens.as_slice()
            .split_inclusive(|t| t == &Token::TripleSep)
            .map(|tokens| Token::vec_to_string(tokens.to_vec()))
            .filter(|triple| !(triple.starts_with("@base") || triple.starts_with("@prefix") || triple.is_empty()))
            .collect();

        let triples: Vec<Triple> = full_triples.into_iter().flat_map(|t| {
            // Trim leading and trainling whitespace
            Self::triple(t.trim()).unwrap()
        }).collect();

        Ok(Graph {
            base_prefix,
            prefixes,
            triples
        })
    }
}
