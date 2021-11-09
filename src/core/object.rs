use crate::core::{ Uri, uri::UriType };

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Literal {
    pub value: String,
    pub datatype: Uri,
    pub language: Option<String>,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        if let Some(language) = &self.language {
            format!("{}^^{}@{}", self.value, self.datatype.to_string(), language)
        } else {
            format!("{}^^{}", self.value, self.datatype.to_string())
        }
    }
}

impl From<&str> for Literal {
    fn from(l: &str) -> Self {
        Literal {
            value: l.into(),
            datatype: Uri {
                prefix: "xsd:".into(),
                name: "string".into(),
                uri_type: UriType::Prefixed
            },
            language: None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Object {
    Literal(Literal),
    Resource(Uri)
}

impl Object {
    pub fn is_literal(&self) -> bool {
        match &self {
            Object::Literal(_) => true,
            _ => false
        }
    }
    
    pub fn is_resource(&self) -> bool {
        match &self {
            Object::Resource(_) => true,
            _ => false
        }
    }

    pub fn literal(&self) -> Option<&Literal> {
        match &self {
            Object::Literal(l) => Some(&l),
            _ => None
        }
    }
    
    pub fn resource(&self) -> Option<&Uri> {
        match &self {
            Object::Resource(r) => Some(&r),
            _ => None
        }
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match &self {
            Object::Literal(literal) => literal.to_string(),
            Object::Resource(resource) => resource.to_string()
        }
    }
}

pub mod matches {
    use regex::Regex;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref WITH_DATATYPE: Regex = Regex::new(r"(.+)\^\^(.+)").unwrap();
        pub static ref WITH_LANG: Regex = Regex::new(r"(.+)@(.{2,5})$").unwrap();
    }
}
