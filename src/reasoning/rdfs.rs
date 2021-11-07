use crate::reasoning::{ BaseReasoner, Entailment };
use crate::core::Triple;
use crate::parsing::{ BaseParser, TurtleParser };

pub struct RDFSReasoner;
impl BaseReasoner for RDFSReasoner {
    fn get_entailment_patterns() -> Vec<Entailment> {
        // https://www.w3.org/TR/rdf11-mt
        // Section 9.2.1

        let rdfs1 = Entailment {
            input_length: 1,
            output_length: 2,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    triples[0].object.is_literal()
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object.literal().unwrap();

                    TurtleParser::triple(&format!("{} {} [ rdf:type {} ] .",
                        subject.to_string(), predicate.to_string(), object.datatype.to_string()
                    )).unwrap()
                }
            )
        };
        
        let rdfs2 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:domain" {
                        let subject_a = &triples[0].subject;
                        let object_a = &triples[0].object;

                        predicate_b.to_string() == subject_a.to_string() &&
                            object_a.is_resource()
                    } else if predicate_b.to_string() == "rdfs:domain" {
                        let subject_b = &triples[1].subject;
                        let object_b = &triples[1].object;

                        predicate_a.to_string() == subject_b.to_string() &&
                            object_b.is_resource()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:domain" {
                        let subject_b = &triples[1].subject;
                        let object_a = &triples[0].object.resource().unwrap();

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            subject_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else if predicate_b.to_string() == "rdfs:domain" {
                        let subject_a = &triples[0].subject;
                        let object_b = &triples[1].object.resource().unwrap();

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            subject_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs3 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:range" {
                        let subject_a = &triples[0].subject;
                        let object_a = &triples[0].object;

                        predicate_b.to_string() == subject_a.to_string() &&
                            object_a.is_resource()
                    } else if predicate_b.to_string() == "rdfs:range" {
                        let subject_b = &triples[1].subject;
                        let object_b = &triples[1].object;

                        predicate_a.to_string() == subject_b.to_string() &&
                            object_b.is_resource()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:range" {
                        let object_b = &triples[1].object.resource().unwrap();
                        let object_a = &triples[0].object.resource().unwrap();

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            object_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else if predicate_b.to_string() == "rdfs:range" {
                        let object_a = &triples[0].object.resource().unwrap();
                        let object_b = &triples[1].object.resource().unwrap();

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            object_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs4 = Entailment {
            input_length: 1,
            output_length: 2,
            input_pattern: Box::new(
                |_| {
                    true
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;
                    let object = &triples[0].object;

                    TurtleParser::graph(&format!(
                        "{} rdf:type rdfs:Resource .\
                         {} rdf:type rdfs:Resource .",
                        subject.to_string(), object.to_string()
                    )).unwrap().triples
                }
            )
        };
        
        let rdfs5 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject_a = &triples[0].subject;
                    let subject_b = &triples[1].subject;

                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    let object_a = &triples[0].object;
                    let object_b = &triples[1].object;
                    
                    (predicate_a.to_string() == "rdfs:subPropertyOf" && predicate_b.to_string() == "rdfs:subPropertyOf") &&
                        (object_a.to_string() == subject_b.to_string() || object_b.to_string() == subject_a.to_string())
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject_b = &triples[1].subject;
                    let object_a = &triples[0].object;
                    
                    let subject_a = &triples[0].subject;
                    let object_b = &triples[1].object;

                    if subject_b.to_string() == object_a.to_string() {
                        TurtleParser::triple(&format!("{} rdfs:subPropertyOf {} .", 
                            subject_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else if subject_a.to_string() == object_b.to_string() {
                        TurtleParser::triple(&format!("{} rdfs:subPropertyOf {} .",
                            subject_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs6 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object;

                    predicate.to_string() == "rdf:type" &&
                        object.to_string() == "rdf:Property"
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subPropertyOf {0} .", 
                        subject.to_string()
                    )).unwrap()
                }
            )
        };

        let rdfs7 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    let object_a = &triples[0].object;
                    let object_b = &triples[1].object;

                    if predicate_a.to_string() == "rdfs:subPropertyOf" {
                        let subject_a = &triples[0].subject;

                        subject_a.to_string() == predicate_b.to_string() &&
                            object_a.is_resource() && object_b.is_resource()
                    } else if predicate_b.to_string() == "rdfs:subPropertyOf" {
                        let subject_b = &triples[1].subject;

                        subject_b.to_string() == predicate_a.to_string() &&
                            object_a.is_resource() && object_b.is_resource()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;
                    
                    let object_a = &triples[0].object;
                    let object_b = &triples[1].object;

                    if predicate_a.to_string() == "rdfs:subPropertyOf" {
                        let subject_b = &triples[1].subject;

                        TurtleParser::triple(&format!("{} {} {} .", 
                            subject_b.to_string(), object_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else if predicate_b.to_string() == "rdfs:subProperyOf" {
                        let subject_a = &triples[0].subject;

                        TurtleParser::triple(&format!("{} {} {} .", 
                            subject_a.to_string(), object_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs8 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object;

                    predicate.to_string() == "rdf:type" &&
                        object.to_string() == "rdfs:Class"
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf rdfs:Resource .", 
                        subject.to_string()
                    )).unwrap()
                }
            )
        };
        
        let rdfs9 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subClassOf" && predicate_b.to_string() == "rdf:type" {
                        let subject_a = &triples[0].subject;
                        let object_b = &triples[1].object;

                        subject_a.to_string() == object_b.to_string()
                    } else if predicate_b.to_string() == "rdfs:subClassOf" && predicate_a.to_string() == "rdf:type" {
                        let subject_b = &triples[1].subject;
                        let object_a = &triples[0].object;

                        subject_b.to_string() == object_a.to_string()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subClassOf" {
                        let object_a = &triples[0].object;
                        let subject_b = &triples[1].subject;

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            subject_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else if predicate_b.to_string() == "rdfs:subClassOf" {
                        let object_b = &triples[1].object;
                        let subject_a = &triples[0].subject;

                        TurtleParser::triple(&format!("{} rdf:type {} .", 
                            subject_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs10 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object;

                    predicate.to_string() == "rdf:type" &&
                        object.to_string() == "rdfs:Class"
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf {0} .", 
                        subject.to_string()
                    )).unwrap()
                }
            )
        };
        
        let rdfs11 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject_a = &triples[0].subject;
                    let subject_b = &triples[1].subject;
                    
                    let predicate_a = &triples[0].predicate;
                    let predicate_b = &triples[1].predicate;
                    
                    let object_a = &triples[0].object;
                    let object_b = &triples[1].object;
                    
                    (predicate_a.to_string() == "rdfs:subClassOf" && predicate_a.to_string() == predicate_b.to_string()) &&
                        (object_a.to_string() == subject_b.to_string() || object_b.to_string() == subject_a.to_string())
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject_b = &triples[1].subject;
                    let object_a = &triples[0].object;
                    
                    let subject_a = &triples[0].subject;
                    let object_b = &triples[1].object;

                    if subject_b.to_string() == object_a.to_string() {
                        TurtleParser::triple(&format!("{} rdfs:subClassOf {} .", 
                            subject_a.to_string(), object_b.to_string()
                        )).unwrap()
                    } else if subject_a.to_string() == object_b.to_string() {
                        TurtleParser::triple(&format!("{} rdfs:subClassOf {} .", 
                            subject_b.to_string(), object_a.to_string()
                        )).unwrap()
                    } else {
                        panic!("Invalid entailment.")
                    }
                }
            )
        };
        
        let rdfs12 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object;

                    predicate.to_string() == "rdf:type" &&
                        object.to_string() == "rdfs:ContainerMembershipProperty"
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subPropertyOf rdfs:member .", 
                        subject.to_string()
                    )).unwrap()
                }
            )
        };
        
        let rdfs13 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let predicate = &triples[0].predicate;
                    let object = &triples[0].object;

                    predicate.to_string() == "rdf:type" &&
                        object.to_string() == "rdfs:Datatype"
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let subject = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf rdfs:Literal .", 
                        subject.to_string()
                    )).unwrap()
                }
            )
        };

        vec![
            rdfs1 , rdfs2 , rdfs3 ,
            rdfs4 , rdfs5 , rdfs6 ,
            rdfs7 , rdfs8 , rdfs9 ,
            rdfs10, rdfs11, rdfs12,
            rdfs13
        ]
    }
}
