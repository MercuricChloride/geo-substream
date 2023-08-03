//! This module contains definitions for geo triples and helpers for working with them.
//!
//! Triples were confusing to me when I was first using them. So I'm going to try to explain them here.
//!
//! You can use the terms Triples and Edges interchangeably.
//! You can also use the terms Entity and Node interchangeably.
//!
//! They are used to show relationships between things.
//! So if I want to show that I own a car, I would create a triple that looks like this:
//!
//! `(Me, Owns, Car)`
//!
//! Where everything within a triple is an Entity.
//! So I would have an Entity for Me, an Entity for Owns, and an Entity for Car.
//! And then I would create a triple that connects them.
//!
//! However, there is an important distinction to make here.
//! In geo we also have triples that have primitive "values"
//! So if I wanted to show that I am 22 years old, I could create a triple that looks like this:
//!
//! `(Me, YearsOld, 22)`

use serde::{Deserialize, Serialize};

// An action is a collection of action triples, this is used to represent a change to the graph.
#[derive(Serialize, Deserialize, Debug)]
pub struct Action<'a> {
    /// Tbh I'm not sure why this is called type, but it is.
    #[serde(rename = "type")]
    pub action_type: &'a str,
    /// ???
    pub version: &'a str,
    /// The collection of action triples that make up this action.
    pub actions: Vec<ActionTriple<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionTriple<'a> {
    #[serde(rename = "type")]
    pub action_triple_type: ActionTripleType,
    pub entity_id: &'a str,
    pub attribute_id: &'a str,
    pub value: ActionTripleValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionTripleValue {
    #[serde(rename = "type")]
    pub value_type: ValueType,
    pub value: String,
    pub id: String,
}

/// In geo we have a concept of actions, which represent changes to make in the graph.
/// This enum represents the different types of actions that can be taken.
#[derive(Serialize, Deserialize, Debug)]
pub enum ActionTripleType {
    /// This is used to create a new entity.
    Create,
    /// This is used to update an existing triple.
    Update,
    /// This is used to delete an existing entity.
    Delete,
}

/// An Entity in geo is a node in the graph that has a unique identifier.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity(pub String);

/// This represents a triple in the graph. (Entity, Attribute, Value)
/// Where the Entity is the node that the triple is connected to.
/// The Attribute is the type of relationship that the triple represents. (Which is also an entity / node in the graph)
/// And the Value is the value of the triple. (Either an entity or a primitive value)
#[derive(Serialize, Deserialize, Debug)]
pub struct Triple {
    pub entity: Entity,
    pub attribute: Entity,
    pub value: ValueType,
}

/// This represents the value type of a triple. IE The final part of a triple. (Entity, Attribute, _Value_)
#[derive(Serialize, Deserialize, Debug)]
pub enum ValueType {
    String(String),
    Number(i64),
    Entity(Entity),
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn can_serialize_mock_data() {
        let mock_data = fs::read_to_string("../mock_data.json").unwrap();
        let action: Action = serde_json::from_str(&mock_data).unwrap();
        println!("{:?}", action);
    }
}

// pub fn handle_action_triple(action_triple: ActionTripleType) {
//     match action_triple {
//         ActionTripleType::Create(entity) => {
//             println!("Create: {:?}", entity);
//         }
//         ActionTripleType::Update(triple) => {
//             println!("Update: {:?}", triple);
//         }
//         ActionTripleType::Delete(entity) => {
//             println!("Delete: {:?}", entity);
//         }
//     }
// }
