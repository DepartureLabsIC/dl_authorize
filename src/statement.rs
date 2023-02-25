use crate::effect::Effect;
use crate::request::Request;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};

/// Enum representing the identity of a user
#[derive(Serialize, Deserialize)]
pub enum Identity {
    Principal(Principal),
}

#[derive(Serialize, Deserialize)]
pub enum StatementIdentity {
    Identity(Identity),
    Any,
}

impl StatementIdentity {
    pub fn matches(&self, v: &Identity) -> bool {
        match (self, v) {
            (Self::Any, _) => true,
            (Self::Identity(Identity::Principal(p1)), Identity::Principal(p2)) => p1 == p2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestResource {
    Resource(String),
    Nested {
        node: String,
        next: Option<Box<RequestResource>>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatementResource {
    Resource(String),
    Nested {
        node: String,
        next: Vec<StatementResource>,
    },
}

impl StatementResource {
    pub fn get_node_name(&self) -> &String {
        match self {
            StatementResource::Resource(r) => r,
            StatementResource::Nested { node, next } => node,
        }
    }

    pub fn add_nested(mut self, nested: StatementResource) -> Self {
        match &mut self {
            StatementResource::Resource(node) => StatementResource::Nested {
                node: node.clone(),
                next: vec![nested],
            },
            StatementResource::Nested { node, next } => {
                next.push(nested);
                self
            }
        }
    }

    pub fn add_nested_resources(mut self, nested: Vec<StatementResource>) -> Self {
        if nested.is_empty() {
            return self;
        }
        match &mut self {
            StatementResource::Resource(node) => StatementResource::Nested {
                node: node.clone(),
                next: nested,
            },
            StatementResource::Nested { node, next } => {
                next.extend(nested);
                self
            }
        }
    }

    pub fn matches(&self, request: &RequestResource) -> bool {
        match (self, request) {
            (Self::Resource(v), RequestResource::Nested { node, next }) => {
                return v == node && next.is_none();
            }
            (Self::Resource(left), RequestResource::Resource(right)) => left == right,
            (Self::Nested { node, next }, RequestResource::Resource(r)) => {
                return next.is_empty() && node == r;
            }
            (
                Self::Nested {
                    node: node_left,
                    next: next_left,
                },
                RequestResource::Nested {
                    node: node_right,
                    next: next_right,
                },
            ) => {
                if node_left != node_right {
                    return false;
                }
                for left in next_left {
                    for right in next_right {
                        if left.matches(right) {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Statement {
    effect: Effect,
    identities: Vec<StatementIdentity>,
    operations: Vec<String>,
    resources: Vec<StatementResource>,
}

impl Statement {
    pub fn new(
        effect: Effect,
        identities: Vec<StatementIdentity>,
        operations: Vec<String>,
        resources: Vec<StatementResource>,
    ) -> Self {
        Statement {
            effect,
            identities,
            operations,
            resources,
        }
    }

    pub fn get_effect(&self, request: &Request) -> Option<Effect> {
        let identity = Identity::Principal(request.caller().clone());

        if !self.operations.contains(&request.action()) {
            return None;
        }

        let identity_match_maybe = self.identities.iter().find(|v| v.matches(&identity));

        if identity_match_maybe.is_none() {
            return None;
        }

        let resource_match_maybe = self
            .resources
            .iter()
            .find(|v| v.matches(&request.resource()));

        match (identity_match_maybe, resource_match_maybe) {
            (Some(_), Some(_)) => Some(self.effect.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod resource_statement_tests {
    use crate::request::RequestResourceBuilder;
    use crate::statement::{RequestResource, StatementResource};

    #[test]
    fn it_builds_a_request_resource() {
        let request_resource = RequestResourceBuilder::new("foo")
            .add("bar")
            .add("baz")
            .build();

        let mut expected = vec!["foo", "bar", "baz"];

        fn c(l: Box<RequestResource>, mut expected: Vec<&str>) {
            match *l {
                RequestResource::Resource(v) => {
                    assert!(v == expected.remove(0))
                }
                RequestResource::Nested { node, next } => {
                    assert_eq!(node, expected.remove(0));
                    c(next.unwrap(), expected)
                }
            }
        }

        c(Box::new(request_resource), expected);
    }

    #[test]
    fn it_builds_a_statement_resource() {
        let v = StatementResource::Resource("Foo".to_string())
            .add_nested(StatementResource::Resource("Bar".to_string()))
            .add_nested(
                StatementResource::Resource("Baz".to_string()).add_nested_resources(vec![
                    StatementResource::Resource("Fizz".to_string()),
                    StatementResource::Resource("Fuzz".to_string()),
                ]),
            );
        println!("{:?}", v);
    }

    #[test]
    fn it_matches_a_request_to_a_statement() {
        let statement = StatementResource::Resource("Foo".to_string()).add_nested(
            StatementResource::Resource("Bar".to_string())
                .add_nested(StatementResource::Resource("Baz".to_string())),
        );
        assert!(statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Baz")
                .build()
        ));
        assert!(!statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Fizz")
                .build()
        ));
    }

    #[test]
    fn it_matches_a_request_to_a_nested_statement() {
        let statement = StatementResource::Resource("Foo".to_string()).add_nested(
            StatementResource::Resource("Bar".to_string())
                .add_nested(StatementResource::Resource("Baz".to_string()))
                .add_nested(StatementResource::Resource("Fizz".to_string())),
        );
        assert!(statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Baz")
                .build()
        ));
        assert!(statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Fizz")
                .build()
        ));
    }

    #[test]
    fn it_matches_a_request_to_a_double_nexted_statement() {
        let statement = StatementResource::Resource("Foo".to_string()).add_nested(
            StatementResource::Resource("Bar".to_string())
                .add_nested(StatementResource::Resource("Baz".to_string()))
                .add_nested(
                    StatementResource::Resource("Fizz".to_string())
                        .add_nested(StatementResource::Resource("Buzz".to_string())),
                ),
        );
        assert!(statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Baz")
                .build()
        ));
        assert!(!statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Fizz")
                .build()
        ));
        assert!(statement.matches(
            &RequestResourceBuilder::new("Foo")
                .add("Bar")
                .add("Fizz")
                .add("Buzz")
                .build()
        ));
    }
}
