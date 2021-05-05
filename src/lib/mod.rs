use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub name: String,
    pub body: String,
}

impl Alias {
    pub fn to_bash(&self) -> String {
        format!(r#"alias {} = "{}""#, self.name, self.body)
    }

    pub fn to_nu(&self) -> String {
        format!(r#""alias {} = {}""#, self.name, self.body)
    }
}