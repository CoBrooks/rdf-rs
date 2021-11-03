use crate::core::Uri;

#[derive(Debug, Clone, PartialEq)]
pub struct Resource(pub Uri);

impl From<Uri> for Resource {
    fn from(u: Uri) -> Self {
        Self(u)
    }
}

impl ToString for Resource {
    fn to_string(&self) -> String {
        format!("{}", self.0.to_string())
    }
}

