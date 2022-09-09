use wright::*;

fn main() {
    describe("rdf_rs", || {
        describe("URI", || {
            use rdf_rs::URI;

            describe("::new_absolute(uri)", || {
                it("should create a new absolute URI", || {
                    let uri = URI::new_absolute("<http://xmlns.com/foaf/0.1/Person>");

                    expect(&uri).to().be().ok()
                });
                
                it("should fail if input is empty", || {
                    let uri = URI::new_absolute("");

                    expect(&uri).to().be().err().and(
                        expect(&uri).when().err_unwrapped().to().equal(
                            rdf_rs::Error::NoneOrEmptyParam("uri".to_string())
                        )
                    )
                });
                
                it("should fail when given invalid URI", || {
                    let uri = URI::new_absolute("foo:bar");

                    expect(&uri).to().be().err().and(
                        expect(&uri).when().err_unwrapped().to().equal(
                            rdf_rs::Error::NotAbsoluteURI
                        )
                    )
                });
            });
            
            describe("::new_prefixed(prefix, resource)", || {
                it("should create a new prefixed URI", || {
                    let uri = URI::new_prefixed("foaf", "Person");

                    expect(&uri).to().be().ok()
                });
            });
            
            describe("::parse(input)", || {
                it("should return an absolute URI when input = <http://xmlns.com/foaf/0.1/Person>", || {
                    let uri = URI::parse("<http://xmlns.com/foaf/0.1/Person>");

                    expect(&uri).to().be().ok()
                    // TODO: expect URI::Absolute
                });
                
                it("should return a prefixed URI when input = foaf:Person", || {
                    let uri = URI::parse("foaf:Person");

                    expect(&uri).to().be().ok()
                    // TODO: expect URI::Prefixed
                });
                
                it("should fail when given invalid URI", || {
                    let uri = URI::parse("Hello, World!");

                    expect(&uri).to().be().err().and(
                        expect(&uri).when().err_unwrapped().to().equal(
                            rdf_rs::Error::ParseError(
                                r#""Hello, World!" is not a valid URI"#.to_string()
                            )
                        )
                    )
                });
            });

        });
        
        describe("Triple", || {
            use rdf_rs::Triple;
            
            describe("::parse(input)", || {
                it("should parse a string into a new Triple", || {
                    let input = "ex:Cole rdf:type <http://xmlns.com/foaf/0.1/Person> .";

                    let triple = Triple::parse(input);

                    expect(&triple).to().be().ok()
                });
            });
        });
        
        describe("Store", || {
            use rdf_rs::{ Store, Triple, URI };

            describe("::new(base, prefixes)", || {
                it("should initialize a new store without base", || {
                    let store = Store::new(None, None);

                    expect(&store).to().be().ok()
                });
                
                it("should initialize a new store with base and prefixes", || {
                    let base = URI::parse("<http://graph.example.com/>").unwrap();
                    let store = Store::new(Some(base), None);

                    expect(&store).to().be().ok()
                });
            });

            describe(".canonicalize_triple(triple)", || {
                let mut store = Store::new(None, None).unwrap();
                let ex = URI::parse("<http://graph.example.com/People#>").unwrap();
                let rdf = URI::parse("<http://www.w3.org/1999/02/22-rdf-syntax-ns#>").unwrap();
                let foaf = URI::parse("<http://xmlns.com/foaf/0.1/>").unwrap();

                store.add_prefix("ex", ex).unwrap();
                store.add_prefix("rdf", rdf).unwrap();
                store.add_prefix("foaf", foaf).unwrap();

                it("should canonicalize all of the URIs in a triple", || {
                    let pre_triple = Triple::parse("ex:Cole rdf:type foaf:Person .").unwrap();

                    let canon_triple = store.canonicalize_triple(&pre_triple);

                    expect(&canon_triple).to().be().ok()
                });
            });

            // describe(".contains(triple)", || {
            //     it("should check for canonical triples", || {

            //     });
            //     
            //     it("should check for prefixed triples", || {

            //     });
            // });
        });
    });
}
