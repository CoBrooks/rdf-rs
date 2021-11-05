use crate::reasoning::{ BaseReasoner, Entailment };
use crate::core::{ Resource, Relationship, Object, Triple };
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
                    if let Object::Literal(_) = &triples[0].object {
                        true
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (Resource(subject), Relationship(predicate), _) = &triples[0].clone().into();
                    let object = if let Object::Literal(l) = &triples[0].object { l } else { panic!() };

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
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if &predicate_a.to_string() == "rdfs:domain" {
                        let Resource(subject) = &triples[0].subject;
                        let Relationship(predicate) = &triples[1].predicate;

                        predicate.to_string() == subject.to_string()
                    } else if &predicate_b.to_string() == "rdfs:domain" {
                        let Resource(subject) = &triples[1].subject;
                        let Relationship(predicate) = &triples[0].predicate;

                        predicate.to_string() == subject.to_string()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    if let Object::Resource(object) = &triples[0].object {
                        let Resource(subject) = &triples[1].subject;

                        TurtleParser::triple(&format!("{} rdf:type {} .", subject.to_string(), object.to_string())).unwrap()
                    } else {
                        Vec::new()
                    }
                }
            )
        };
        
        let rdfs3 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if &predicate_a.to_string() == "rdfs:range" {
                        let Resource(subject) = &triples[0].subject;
                        let Relationship(predicate) = &triples[1].predicate;

                        predicate.to_string() == subject.to_string()
                    } else if &predicate_b.to_string() == "rdfs:range" {
                        let Resource(subject) = &triples[1].subject;
                        let Relationship(predicate) = &triples[0].predicate;

                        predicate.to_string() == subject.to_string()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    if let Object::Resource(object_type) = &triples[0].object {
                        if let Object::Resource(object) = &triples[1].object {
                            TurtleParser::triple(&format!("{} rdf:type {} .", object.to_string(), object_type.to_string())).unwrap()
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
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
                    let (s, _, o) = &triples[0].clone().into();

                    let mut first = TurtleParser::triple(&format!("{} rdf:type rdfs:Resource .", s.to_string())).unwrap();
                    let mut second = TurtleParser::triple(&format!("{} rdf:type rdfs:Resource .", o.to_string())).unwrap();
                    first.append(&mut second);

                    first
                }
            )
        };
        
        let rdfs5 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;
                    
                    if predicate_a.to_string() == "rdfs:subPropertyOf" && predicate_a.to_string() == predicate_b.to_string() {
                        if let Object::Resource(object_a) = &triples[0].object {
                            let Resource(subject_b) = &triples[1].subject;

                            object_a.to_string() == subject_b.to_string()
                        } else if let Object::Resource(object_b) = &triples[1].object {
                            let Resource(subject_a) = &triples[0].subject;

                            object_b.to_string() == subject_a.to_string()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject_b) = &triples[1].subject;
                    let object_a = &triples[0].object;
                    
                    let Resource(subject_a) = &triples[0].subject;
                    let object_b = &triples[1].object;

                    if let Object::Resource(object_a) = object_a {
                        if subject_b.to_string() == object_a.to_string() {
                            let object_b = if let Object::Resource(o) = object_b { o } else { panic!() };

                            TurtleParser::triple(&format!("{} rdfs:subPropertyOf {} .", subject_a.to_string(), object_b.to_string())).unwrap()
                        } else if let Object::Resource(object_b) = object_b {
                            if subject_a.to_string() == object_b.to_string() {
                                TurtleParser::triple(&format!("{} rdfs:subPropertyOf {} .", subject_b.to_string(), object_a.to_string())).unwrap()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                }
            )
        };
        
        let rdfs6 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (_, Relationship(predicate), object) = &triples[0].clone().into();

                    if predicate.to_string() == "rdf:type" {
                        if let Object::Resource(object) = object {
                            object.to_string() == "rdf:Property"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject) = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subPropertyOf {0} .", subject.to_string())).unwrap()
                }
            )
        };

        let rdfs7 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subPropertyOf" {
                        let Resource(subject_a) = &triples[0].subject;

                        subject_a.to_string() == predicate_b.to_string()
                    } else if predicate_b.to_string() == "rdfs:subPropertyOf" {
                        let Resource(subject_b) = &triples[1].subject;

                        subject_b.to_string() == predicate_a.to_string()
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subPropertyOf" {
                        let object_a = if let Object::Resource(o) = &triples[0].object { o } else { panic!() };
                        let object_b = if let Object::Resource(o) = &triples[1].object { o } else { panic!() };
                        let Resource(subject_b) = &triples[1].subject;

                        TurtleParser::triple(&format!("{} {} {} .", subject_b.to_string(), object_a.to_string(), object_b.to_string())).unwrap()
                    } else if predicate_b.to_string() == "rdfs:subProperyOf" {
                        let object_a = if let Object::Resource(o) = &triples[0].object { o } else { panic!() };
                        let object_b = if let Object::Resource(o) = &triples[1].object { o } else { panic!() };
                        let Resource(subject_a) = &triples[0].subject;

                        TurtleParser::triple(&format!("{} {} {} .", subject_a.to_string(), object_b.to_string(), object_a.to_string())).unwrap()
                    } else {
                        Vec::new()
                    }
                }
            )
        };
        
        let rdfs8 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (_, Relationship(predicate), object) = &triples[0].clone().into();

                    if predicate.to_string() == "rdf:type" {
                        if let Object::Resource(object) = object {
                            object.to_string() == "rdfs:Class"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject) = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf rdfs:Resource .", subject.to_string())).unwrap()
                }
            )
        };
        
        let rdfs9 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subClassOf" && predicate_b.to_string() == "rdf:type" {
                        let Resource(subject_a) = &triples[0].subject;

                        if let Object::Resource(object_b) = &triples[1].object {
                            subject_a.to_string() == object_b.to_string()
                        } else {
                            false
                        }
                    } else if predicate_b.to_string() == "rdfs:subClassOf" && predicate_a.to_string() == "rdf:type" {
                        let Resource(subject_b) = &triples[1].subject;

                        if let Object::Resource(object_a) = &triples[0].object {
                            subject_b.to_string() == object_a.to_string()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;

                    if predicate_a.to_string() == "rdfs:subClassOf" {
                        let object_a = if let Object::Resource(o) = &triples[0].object { o } else { panic!() };
                        let Resource(subject_b) = &triples[1].subject;

                        TurtleParser::triple(&format!("{} rdf:type {} .", subject_b.to_string(), object_a.to_string())).unwrap()
                    } else if predicate_b.to_string() == "rdfs:subClassOf" {
                        let object_b = if let Object::Resource(o) = &triples[1].object { o } else { panic!() };
                        let Resource(subject_a) = &triples[0].subject;

                        TurtleParser::triple(&format!("{} rdf:type {} .", subject_a.to_string(), object_b.to_string())).unwrap()
                    } else {
                        Vec::new()
                    }
                }
            )
        };
        
        let rdfs10 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (_, Relationship(predicate), object) = &triples[0].clone().into();

                    if predicate.to_string() == "rdf:type" {
                        if let Object::Resource(object) = object {
                            object.to_string() == "rdfs:Class"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject) = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf {0} .", subject.to_string())).unwrap()
                }
            )
        };
        
        let rdfs11 = Entailment {
            input_length: 2,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Relationship(predicate_a) = &triples[0].predicate;
                    let Relationship(predicate_b) = &triples[1].predicate;
                    
                    if predicate_a.to_string() == "rdfs:subClassOf" && predicate_a.to_string() == predicate_b.to_string() {
                        if let Object::Resource(object_a) = &triples[0].object {
                            let Resource(subject_b) = &triples[1].subject;

                            object_a.to_string() == subject_b.to_string()
                        } else if let Object::Resource(object_b) = &triples[1].object {
                            let Resource(subject_a) = &triples[0].subject;

                            object_b.to_string() == subject_a.to_string()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject_b) = &triples[1].subject;
                    let object_a = &triples[0].object;
                    
                    let Resource(subject_a) = &triples[0].subject;
                    let object_b = &triples[1].object;

                    if let Object::Resource(object_a) = object_a {
                        if subject_b.to_string() == object_a.to_string() {
                            let object_b = if let Object::Resource(o) = object_b { o } else { panic!() };

                            TurtleParser::triple(&format!("{} rdfs:subClassOf {} .", subject_a.to_string(), object_b.to_string())).unwrap()
                        } else if let Object::Resource(object_b) = object_b {
                            if subject_a.to_string() == object_b.to_string() {
                                TurtleParser::triple(&format!("{} rdfs:subClassOf {} .", subject_b.to_string(), object_a.to_string())).unwrap()
                            } else {
                                Vec::new()
                            }
                        } else {
                            Vec::new()
                        }
                    } else {
                        Vec::new()
                    }
                }
            )
        };
        
        let rdfs12 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (_, Relationship(predicate), object) = &triples[0].clone().into();

                    if predicate.to_string() == "rdf:type" {
                        if let Object::Resource(object) = object {
                            object.to_string() == "rdfs:ContainerMembershipProperty"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject) = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subPropertyOf rdfs:member .", subject.to_string())).unwrap()
                }
            )
        };
        
        let rdfs13 = Entailment {
            input_length: 1,
            output_length: 1,
            input_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let (_, Relationship(predicate), object) = &triples[0].clone().into();

                    if predicate.to_string() == "rdf:type" {
                        if let Object::Resource(object) = object {
                            object.to_string() == "rdfs:Datatype"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            ),
            output_pattern: Box::new(
                |triples: &Vec<Triple>| {
                    let Resource(subject) = &triples[0].subject;

                    TurtleParser::triple(&format!("{} rdfs:subClassOf rdfs:Literal .", subject.to_string())).unwrap()
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
