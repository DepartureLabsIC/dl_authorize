use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Effect {
    Allow,
    Deny,
}

impl Eq for Effect {}

impl PartialEq<Self> for Effect {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Allow, Self::Allow) => true,
            (Self::Deny, Self::Deny) => true,
            _ => false,
        }
    }
}

/// Least permissive 0!!
impl PartialOrd<Self> for Effect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Least permissive 0!!
impl Ord for Effect {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = match self {
            Self::Deny => 0,
            _ => 1000
        };

        let right = match other {
            Self::Deny => 0,
            _ => 1000
        };

        left.cmp(&right)
    }
}