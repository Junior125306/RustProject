use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

pub struct St {
    name: String,
    fields: Vec<Fd>,
}

pub struct Fd {
    name: String,
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

impl Schema {
    pub fn into_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(k.as_str(), v))
                    .collect();
                structs.push(St::new(p(self.title.as_ref().unwrap()), fields));
                structs
            }
            _ => panic!("Not supported yet"),
        }
    }
}

fn p(s: &str) -> String {
    todo!()
}

fn n(s: &str) -> String {
    todo!()
}

fn process_type(k: &str, v: &Schema) -> Fd {
    match v.ty.as_str() {
        "object" => {
            todo!()
        }
        "integer" => Fd::new(n(k), "i64"),
        "float" => Fd::new(n(k), "f64"),
        "string" => Fd::new(n(k), "String"),
        _ => panic!("Not supported yet"),
    }
}

mod tests {
    use super::*;
    const PERSON1: &str = include_str!("../fixtures/person.json");
    #[test]
    fn schema_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();
        let mut structs = schema.into_st();
        assert_eq!(structs.len(), 1);
        let st = structs.pop().unwrap();
        assert_eq!(st.name, "Person");
        assert_eq!(st.fields.len(), 2);
        let mut names = st.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        names.sort();
        assert_eq!(&names[..], &["first_name", "last_name"]);
        assert_eq!(st.fields[0].ty, "String");
        assert_eq!(st.fields[1].ty, "String");
    }
}
