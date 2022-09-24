use wright::*;

fn main() {
    describe("rdf_rs", || {
        describe("URI", || {
            use rdf_rs::{ Error, Namespace, URI };

            describe("::new", || {
                it("should create new absolute URIs", || {
                    let uri = URI::parse("<http://graph.example.com/firstName>");

                    let expected = URI::new(
                        Namespace::Absolute("http://graph.example.com/".to_string()),
                        "firstName"
                    );

                    expect(&uri).to().be().ok().and(
                        expect(&uri).when().unwrapped().to().equal(expected)
                    )
                });
                
                it("should create new prefixed URIs", || {
                    let uri = URI::parse("ex:firstName");

                    let expected = URI::new(
                        Namespace::Prefix("ex".to_string()),
                        "firstName"
                    );

                    expect(&uri).to().be().ok().and(
                        expect(&uri).when().unwrapped().to().equal(expected)
                    )
                });
                
                it("should error with invalid input", || {
                    let s = "foobar";

                    let uri = URI::parse(s);

                    expect(&uri).to().be().err().and(
                        expect(&uri).when().err_unwrapped().to().equal(
                            Error::NotAValidURI
                        )
                    )
                });
            });
        });
        
        describe("Triple", || {
            use rdf_rs::{ Triple, URI };
            
            describe("::parse(input)", || {
                it("should parse a string into a new Triple", || {
                    let input = "ex:Cole rdf:type <http://xmlns.com/foaf/0.1/Person> .";

                    let triple = Triple::parse(input);

                    let subject = URI::parse("ex:Cole").unwrap();
                    let predicate = URI::parse("rdf:type").unwrap();
                    let object = URI::parse("<http://xmlns.com/foaf/0.1/Person>").unwrap();

                    let expected = Triple::new(subject, predicate, object);

                    expect(&triple).to().be().ok()/*.and(
                        expect(&triple).when().unwrapped().to().equal(expected)
                    )*/
                });
            });
        });
        
        describe("Store", || {
            use std::collections::HashMap;
            use rdf_rs::{ Error, Namespace, Store, Triple };

            // let base = Namespace::Absolute("http://graph.example.com/".to_string());
            let prefixes: HashMap<String, Namespace> = [
                ("ex".to_string(), Namespace::Absolute("http://graph.example.com/".to_string())),
                ("rdf".to_string(), Namespace::Absolute("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string())),
                ("foaf".to_string(), Namespace::Absolute("http://xmlns.com/foaf/0.1/".to_string()))
            ].into();

            describe(".add_triple", || {
                let mut store: Store = Store::new(None, Some(prefixes.clone()));

                let triple = Triple::parse("ex:Cole rdf:type foaf:Person .").unwrap();
                let result = store.add_triple(triple.clone());

                it("should add a new triple to the store", || {
                    let contains_new_triple = store.contains(&triple);

                    expect(&result).to().be().ok().and(
                        expect(&contains_new_triple).when().unwrapped().to().equal(true)
                    )
                });
                
                let triple = Triple::parse("ex:Cole foo:bar _:asdf .").unwrap();
                let result = store.add_triple(triple);

                it("should fail to add an invalid triple", || {
                    expect(&result).to().be().err().and(
                        expect(&result).when().err_unwrapped().to().equal(
                            Error::UnknownPrefix("foo".to_string())
                        )
                    )
                });
            });
        });
    });
}
