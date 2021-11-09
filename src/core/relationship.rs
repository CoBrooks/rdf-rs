use crate::core::Uri;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relationship(pub Uri);

impl From<Uri> for Relationship {
    fn from(u: Uri) -> Self {
        Self(u)
    }
}

impl ToString for Relationship {
    fn to_string(&self) -> String {
        format!("{}", self.0.to_string())
    }
}

