#[derive(Debug, Clone, PartialEq)]
pub enum UriType {
    Full,
    Relative,
    Prefixed,
    PrefixedWithBase,
    BlankNode
}

#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    pub prefix: String,
    pub name: String,
    pub uri_type: UriType
}

impl Uri {
    pub fn new(prefix: &str, name: &str, uri_type: UriType) -> Self {
        Self {
            prefix: prefix.into(),
            name: name.into(),
            uri_type
        }
    }
}

impl ToString for Uri {
    fn to_string(&self) -> String {
        format!("{}{}", self.prefix, self.name)
    }
}

pub mod matches {
    use regex::Regex;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref FULL_URL: Regex = Regex::new(r"^<?(http://(?:[\w\d\-_]{2,}\.)+[a-z]{2,}/(?:[\w\d\-_/]+)*[#/])([\w\d\-_]+)>?").unwrap();
        pub static ref RELATIVE_URL: Regex = Regex::new(r"^<#?([\w\d\-_]+)>").unwrap();
        pub static ref PREFIXED: Regex = Regex::new(r"^([\w\d\-_]+:)([\w\d\-_]+)").unwrap();
        pub static ref EMPTY_PREFIX: Regex = Regex::new(r"^:([\w\d\-_]+)").unwrap();
    }
}
