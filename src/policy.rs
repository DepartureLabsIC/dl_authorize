use crate::effect::Effect;
use crate::request::Request;
use crate::statement::Statement;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Default)]
pub struct Policy {
    statements: Vec<Statement>,
}

impl Policy {
    fn get_effect(&self, request: &Request) -> Effect {
        let mut effects = vec![];
        for statement in &self.statements {
            match statement.get_effect(request) {
                None => {}
                Some(effect) => effects.push(effect),
            }
        }
        effects.sort();
        effects.get(0).cloned().unwrap_or(Effect::Deny)
    }

    pub fn add_statement(&mut self, statement: Statement) -> () {
        self.statements.push(statement);
    }
}

pub fn get_effect(p: &Policy, r: &Request) -> Effect {
    return p.get_effect(r);
}

impl Storable for Policy {
    fn to_bytes(&self) -> Cow<[u8]> {
        match serde_json::to_vec(&self) {
            Ok(result) => {
                return Cow::from(result.as_slice().to_owned()).to_owned();
            }
            Err(_) => {
                panic!("Failed to serialize!")
            }
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        match serde_json::from_slice::<Self>(&*bytes) {
            Ok(result) => result,
            Err(_) => {
                panic!("Failed to deserialize!")
            }
        }
    }
}

#[cfg(test)]
mod policy_tests {
    use crate::effect::Effect;
    use crate::policy::Policy;
    use crate::request::RequestResourceBuilder;
    use crate::statement::{Identity, Statement, StatementIdentity, StatementResource};
    use candid::Principal;

    use super::*;

    #[test]
    pub fn it_matches_policy() {
        let policy = Policy {
            statements: vec![Statement::new(
                Effect::Allow,
                vec![StatementIdentity::Identity(Identity::Principal(
                    Principal::anonymous(),
                ))],
                vec!["call".to_string()],
                vec![StatementResource::Resource("Foo".to_string())],
            )],
        };

        assert!(
            policy.get_effect(&Request::new(
                "call".to_string(),
                RequestResourceBuilder::new("Foo").build(),
                Principal::anonymous()
            )) == Effect::Allow
        );

        assert!(
            policy.get_effect(&Request::new(
                "call".to_string(),
                RequestResourceBuilder::new("Bar").build(),
                Principal::anonymous()
            )) == Effect::Deny
        );
    }

    #[test]
    pub fn it_selects_least_permissive() {
        let policy = Policy {
            statements: vec![
                Statement::new(
                    Effect::Deny,
                    vec![StatementIdentity::Identity(Identity::Principal(
                        Principal::anonymous(),
                    ))],
                    vec!["call".to_string()],
                    vec![StatementResource::Resource("Foo".to_string())],
                ),
                Statement::new(
                    Effect::Allow,
                    vec![StatementIdentity::Identity(Identity::Principal(
                        Principal::anonymous(),
                    ))],
                    vec!["call".to_string()],
                    vec![StatementResource::Resource("Foo".to_string())],
                ),
                Statement::new(
                    Effect::Allow,
                    vec![StatementIdentity::Identity(Identity::Principal(
                        Principal::anonymous(),
                    ))],
                    vec!["call".to_string()],
                    vec![StatementResource::Resource("Foo".to_string())
                        .add_nested(StatementResource::Resource("Bar".to_string()))],
                ),
            ],
        };

        assert!(
            policy.get_effect(&Request::new(
                "call".to_string(),
                RequestResourceBuilder::new("Foo").build(),
                Principal::anonymous()
            )) == Effect::Deny
        );

        assert!(
            policy.get_effect(&Request::new(
                "call".to_string(),
                RequestResourceBuilder::new("Foo").add("Bar").build(),
                Principal::anonymous()
            )) == Effect::Allow
        );
    }
}
