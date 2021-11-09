#![allow(dead_code, unused_imports)]
use std::collections::HashSet;

use crate::parsing::{ ParserError, TurtleParser, BaseParser };
use crate::querying::QueryBuilder;
use crate::core::Triple;

type TestReturn = Result<(), ParserError>;

#[test]
fn can_query_simple_graph() -> TestReturn {
    let graph = TurtleParser::from_file("test_data/simple.ttl")?;

    let mut spiderman = graph.start_query(2)
        .subject(|s| s == "ex:spiderman")
        .query();
    spiderman.sort();

    let mut expected_triples = TurtleParser::graph(r#"
        ex:spiderman 
            foaf:name 
                "Spiderman", "Человек-паук"@ru, _:blank1 ;
            rel:enemyOf ex:green-goblin ;
            rdf:type rdfs:Resource, foaf:Person .
    "#)?.triples;
    expected_triples.sort();

    assert_eq!(spiderman, expected_triples);

    let name = graph.start_query(2)
        .subject(|s| s == "ex:spiderman")
        .predicate(|p| p == "foaf:name")
        .value().unwrap();
    let name = name.literal().unwrap();

    assert_eq!(name.value, "\"Spiderman\"".to_string());

    Ok(())
}
