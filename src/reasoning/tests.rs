#![allow(dead_code, unused_imports)]
use crate::parsing::{ BaseParser, TurtleParser, ParserError };
use crate::reasoning::{ RDFSReasoner, BaseReasoner };

type TestReturn = Result<(), ParserError>;

#[test]
fn rdfs_reasoning() -> TestReturn {
    let entailment_rules = &RDFSReasoner::get_entailment_patterns();

    // rdfs1
    {
        let triple = TurtleParser::triple("_:a _:b \"5.0\"^^xsd:string .")?;
        let rdfs1 = &entailment_rules[0];
        assert!(rdfs1.verify(&triple));

        let new_triples = rdfs1.apply(&triple);
        let expected_triples = TurtleParser::graph("_:blank1 rdf:type xsd:string . _:a _:b _:blank1 .")?.triples;
        assert_eq!(new_triples, expected_triples);
    }

    // rdfs2
    {
        let graph = TurtleParser::graph("_:a rdfs:domain _:x . _:y _:a _:z .")?;
        let triples = &graph.triples;
        let rdfs2 = &entailment_rules[1];
        assert!(rdfs2.verify(&triples));

        let new_triples = rdfs2.apply(&triples);
        let expected_triple = TurtleParser::triple("_:y rdf:type _:x .")?;
        assert_eq!(new_triples, expected_triple);
    }

    // rdfs3
    {
        let graph = TurtleParser::graph("_:a rdfs:range _:x . _:y _:a _:z .")?;
        let triples = &graph.triples;
        let rdfs3 = &entailment_rules[2];
        assert!(rdfs3.verify(&triples));

        let new_triples = rdfs3.apply(&triples);
        let expected_triple = TurtleParser::triple("_:z rdf:type _:x .")?;
        assert_eq!(new_triples, expected_triple);
    }
    
    // rdfs4
    {
        let triple = TurtleParser::triple("_:x _:a _:y .")?;
        let rdfs4 = &entailment_rules[3];
        assert!(rdfs4.verify(&triple));

        let new_triples = rdfs4.apply(&triple);
        let expected_triples = TurtleParser::graph("_:x rdf:type rdfs:Resource . _:y rdf:type rdfs:Resource .")?.triples;
        assert_eq!(new_triples, expected_triples);
    }

    // rdfs5
    {
        let triples = TurtleParser::graph("_:x rdfs:subPropertyOf _:y . _:y rdfs:subPropertyOf _:z .")?.triples;
        let rdfs5 = &entailment_rules[4];
        assert!(rdfs5.verify(&triples));

        let new_triple = rdfs5.apply(&triples);
        let expected_triple = TurtleParser::triple("_:x rdfs:subPropertyOf _:z .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs6
    {
        let triple = TurtleParser::triple("_:x rdf:type rdf:Property .")?;
        let rdfs6 = &entailment_rules[5];
        assert!(rdfs6.verify(&triple));

        let new_triple = rdfs6.apply(&triple);
        let expected_triple = TurtleParser::triple("_:x rdfs:subPropertyOf _:x .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs7
    {
        let triples = TurtleParser::graph("_:a rdfs:subPropertyOf _:b . _:x _:a _:y .")?.triples;
        let rdfs7 = &entailment_rules[6];
        assert!(rdfs7.verify(&triples));

        let new_triple = rdfs7.apply(&triples);
        let expected_triple = TurtleParser::triple("_:x _:b _:y .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs8
    {
        let triple = TurtleParser::triple("_:x rdf:type rdfs:Class .")?;
        let rdfs8 = &entailment_rules[7];
        assert!(rdfs8.verify(&triple));

        let new_triple = rdfs8.apply(&triple);
        let expected_triple = TurtleParser::triple("_:x rdfs:subClassOf rdfs:Resource .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs9
    {
        let triples = TurtleParser::graph("_:x rdfs:subClassOf _:y . _:z rdf:type _:x .")?.triples;
        let rdfs9 = &entailment_rules[8];
        assert!(rdfs9.verify(&triples));

        let new_triple = rdfs9.apply(&triples);
        let expected_triple = TurtleParser::triple("_:z rdf:type _:y .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs10
    {
        let triple = TurtleParser::triple("_:x rdf:type rdfs:Class .")?;
        let rdfs10 = &entailment_rules[9];
        assert!(rdfs10.verify(&triple));

        let new_triple = rdfs10.apply(&triple);
        let expected_triple = TurtleParser::triple("_:x rdfs:subClassOf _:x .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs11
    {
        let triples = TurtleParser::graph("_:x rdfs:subClassOf _:y . _:y rdfs:subClassOf _:z .")?.triples;
        let rdfs11 = &entailment_rules[10];
        assert!(rdfs11.verify(&triples));

        let new_triple = rdfs11.apply(&triples);
        let expected_triple = TurtleParser::triple("_:x rdfs:subClassOf _:z .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs12
    {
        let triple = TurtleParser::triple("_:x rdf:type rdfs:ContainerMembershipProperty .")?;
        let rdfs12 = &entailment_rules[11];
        assert!(rdfs12.verify(&triple));

        let new_triple = rdfs12.apply(&triple);
        let expected_triple = TurtleParser::triple("_:x rdfs:subPropertyOf rdfs:member .")?;
        assert_eq!(new_triple, expected_triple);
    }
    
    // rdfs13
    {
        let triple = TurtleParser::triple("_:x rdf:type rdfs:Datatype .")?;
        let rdfs13 = &entailment_rules[12];
        assert!(rdfs13.verify(&triple));

        let new_triple = rdfs13.apply(&triple);
        let expected_triple = TurtleParser::triple("_:x rdfs:subClassOf rdfs:Literal .")?;
        assert_eq!(new_triple, expected_triple);
    }

    Ok(())
}

#[test]
fn can_apply_entailment_to_graph() -> TestReturn {
    let graph = TurtleParser::graph("ex:employer rdfs:domain foaf:Person ;\
                                        rdfs:range foaf:Organization .
                                    ex:John ex:employer ex:Company .")?;

    let inferred = RDFSReasoner::get_inferred_triples(graph.triples, 1);
    dbg!(inferred);

    assert!(false);

    Ok(())
}

