#![allow(dead_code, unused_imports)]
use std::collections::HashMap;
use crate::core::*;
use crate::core::uri::UriType;
use crate::parsing::{ RDFParser, TurtleParser, ParserError };

type TestReturn = Result<(), ParserError>;

#[test]
fn can_parse_uris() -> TestReturn {
    let full_url = r"<http://foo.example.com/bar/person>";
    let relative_url = r"<person>";
    let prefixed = "ex:person";
    let empty_prefix = ":person";

    let expected_uri = Uri::new(r"http://foo.example.com/bar/", "person", UriType::Full);
    assert_eq!(TurtleParser::uri(full_url)?, expected_uri);

    let expected_uri = Uri::new("", "person", UriType::Relative);
    assert_eq!(TurtleParser::uri(relative_url)?, expected_uri);
    
    let expected_uri = Uri::new("ex:", "person", UriType::Prefixed);
    assert_eq!(TurtleParser::uri(prefixed)?, expected_uri);
    
    let expected_uri = Uri::new("", "person", UriType::PrefixedWithBase);
    assert_eq!(TurtleParser::uri(empty_prefix)?, expected_uri);

    Ok(())
}

#[test]
fn can_parse_basic_triple() -> TestReturn {
    let triple = r#"<http://example.com/foo#John> foaf:lastName "Johnson" ."#;

    let ex_subject: Resource = Uri::new("http://example.com/foo#", "John", UriType::Full).into();
    let ex_predicate: Relationship = Uri::new("foaf:", "lastName", UriType::Prefixed).into();
    let ex_object: Object = Object::Literal("\"Johnson\"".into());

    let t: Vec<Triple> = TurtleParser::triple(triple)?;
    assert_eq!(t.len(), 1);

    let t = t[0].clone();
    let expected_t: Triple = (ex_subject, ex_predicate, ex_object).into();
    assert_eq!(t, expected_t);

    Ok(())
}

#[test]
fn can_parse_predicate_list() -> TestReturn {
    let triple = r#"<http://example.com/foo#John> foaf:lastName "Johnson" ; foaf:name "John" ; foaf:email "john@example.com" ."#;

    let triples: Vec<Triple> = TurtleParser::triple(triple)?;
    let expected_triples: Vec<Triple> = vec![
        TurtleParser::triple("<http://example.com/foo#John> foaf:lastName \"Johnson\" .")?,
        TurtleParser::triple("<http://example.com/foo#John> foaf:name \"John\" .")?,
        TurtleParser::triple("<http://example.com/foo#John> foaf:email \"john@example.com\" .")?,
    ].into_iter().flatten().collect();

    assert_eq!(triples, expected_triples);

    Ok(())
}

#[test]
fn can_parse_object_list() -> TestReturn {
    let triple = r#"<http://example.com/foo#John> foaf:goesBy "John", "John Jackson" ."#;

    let triples: Vec<Triple> = TurtleParser::triple(triple)?;
    let expected_triples: Vec<Triple> = vec![
        TurtleParser::triple("<http://example.com/foo#John> foaf:goesBy \"John\" .")?,
        TurtleParser::triple("<http://example.com/foo#John> foaf:goesBy \"John Jackson\" .")?,
    ].into_iter().flatten().collect();

    assert_eq!(triples, expected_triples);

    Ok(())
}

#[test]
fn can_parse_meta() -> TestReturn {
    let rdf_string = "@base <http://example.org/> . @prefix ex: <http://example.org/> . @prefix foo: <http://bar.com/> .";

    let graph = TurtleParser::graph(rdf_string)?;
    assert_eq!(graph.base_prefix.unwrap(), "http://example.org/".to_string());
    
    let expected_prefixes: HashMap<String, String> = HashMap::from([
        ("ex:".to_string(), "http://example.org/".to_string()),
        ("foo:".to_string(), "http://bar.com/".to_string()),
        ("rdf:".to_string(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()),
        ("xsd:".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
    ]);
    assert_eq!(graph.prefixes, expected_prefixes);

    Ok(())
}

#[test]
fn can_apply_meta() -> TestReturn {
    // https://w3.org/TR/turtle Example 1
    let mut graph = TurtleParser::from_file("./test_data/simple.ttl")?;
    graph.apply_metadata();
    
    assert_eq!(graph.triples.len(), 7);

    let triples = graph.triples.clone();
    let unapplied_meta = triples.iter().find(|t| {
        t.subject.0.uri_type != UriType::Full || 
        if let Object::Resource(obj) = &t.object { obj.uri_type != UriType:: Full } else { false } || 
        t.predicate.0.uri_type != UriType::Full 
    }).is_some();
    assert!(!unapplied_meta);

    Ok(())
}

#[test]
fn can_parse_blank_nodes_lists() -> TestReturn {
    // https://w3.org/TR/turtle Example 16
    let graph = TurtleParser::from_file("./test_data/blank_property_list.ttl")?;
    assert_eq!(graph.triples.len(), 6);

    let expected_triples: Vec<Triple> = vec![
        TurtleParser::triple("_:blank1 foaf:name \"Alice\" .")?,
        TurtleParser::triple("_:blank2 foaf:name \"Bob\" .")?,
        TurtleParser::triple("_:blank3 foaf:name \"Eve\" .")?,
        TurtleParser::triple("_:blank2 foaf:knows _:blank3 .")?,
        TurtleParser::triple("_:blank2 foaf:mbox \"bob@example.com\" .")?,
        TurtleParser::triple("_:blank1 foaf:knows _:blank2 .")?,
    ].into_iter().flatten().collect();
    assert_eq!(graph.triples, expected_triples);

    Ok(())
}

#[test]
fn can_parse_collections() -> TestReturn {
    // https://w3.org/TR/turtle/ Example 20
    let triples = TurtleParser::triple(r#":a :b ( "apple" "banana" ) ."#)?;
    assert_eq!(triples.len(), 5);

    let expected_triples: Vec<Triple> = vec![
        TurtleParser::triple("_:blank1 rdf:first \"apple\" .")?,
        TurtleParser::triple("_:blank2 rdf:first \"banana\" .")?,
        TurtleParser::triple("_:blank2 rdf:rest rdf:nil .")?,
        TurtleParser::triple("_:blank1 rdf:rest _:blank2 .")?,
        TurtleParser::triple(":a :b _:blank1 .")?,
    ].into_iter().flatten().collect();
    assert_eq!(triples, expected_triples);
    
    Ok(())
}

#[test]
fn can_parse_literals() -> TestReturn {
    let triples = TurtleParser::triple(r#"_:a _:b "a literal"@en, "-5"^^xsd:integer, true . "#)?;
    assert_eq!(triples.len(), 3);

    let expected_literals = vec![
        Literal { value: "\"a literal\"".to_string(), datatype: TurtleParser::uri("xsd:string")?, language: Some("en".to_string()) },
        Literal { value: "\"-5\"".to_string(), datatype: TurtleParser::uri("xsd:integer")?, language: None },
        Literal { value: "true".to_string(), datatype: TurtleParser::uri("xsd:boolean")?, language: None },
    ];

    let literals: Vec<Literal> = triples.into_iter()
        .map(|t| if let Object::Literal(object) = &t.object { object.clone() } else { panic!("Object is not a literal") })
        .collect();
    assert_eq!(literals, expected_literals);

    Ok(())
}

#[test]
fn can_parse_owl_file() -> TestReturn {
    let graph = TurtleParser::from_file("owl")?;
    dbg!(&graph);
    
    assert!(false);
    Ok(())
}

