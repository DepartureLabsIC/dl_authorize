# Departure Labs Authorize 


**Supports [Departure Labs DLIP 3](https://github.com/DepartureLabsIC/DLIP/blob/main/DLIP3.md)**

This is a library for creating and evaluating resource based policies.

The core components of a policy are `Effect`, `Statement`, `Request`, and `Policy`:

- `Effect`: An enum representing the effect of a statement on a request
- `Statement`: A set of conditions (e.g. identity, resource, operation) that determine the effect of a request
- `Request`: An object containing information about the action, resource and caller of a request
- `Policy`: A collection of statements

## How it works

A `Policy` is a set of rules, represented as a list of `Statement` objects, used to determine whether a `Request` should be authorized or not. Each `Statement` has an `Effect` which specifies whether the statement allows or denies access to a particular resource. When multiple statements apply to the same Request, the policy **must** select the `Effect` from the __least permissive statement__.

By using a `Policy` and a `Request` together, this authorization system can evaluate whether a particular request should be authorized or not based on the rules set forth in the `Policy`.


## Usage

### Creating a Policy

Create a policy by instantiating a `Policy` and adding `Statement`s to it:

```rust
let mut policy = Policy::default();

let statement1 = Statement::new(
    Effect::Allow,
    vec![StatementIdentity::Any],
    vec!["read".to_string()],
    vec![StatementResource::Resource("/path/to/resource".to_string())],
);

policy.add_statement(statement1);

let statement2 = Statement::new(
    Effect::Deny,
    vec![StatementIdentity::Identity(Principal::User("bob".to_string()))],
    vec!["write".to_string(), "delete".to_string()],
    vec![StatementResource::Resource("/path/to/resource".to_string())],
);

policy.add_statement(statement2);
```

### Evaluating a Policy

Evaluate a policy by instantiating a `Request` and passing it to the `Policy::get_effect` method:

```rust
let request = Request::new(
    "read".to_string(),
    RequestResourceBuilder::new("/path/to/resource").build(),
    Principal::User("bob".to_string()),
);
let effect = policy.get_effect(&request);
assert_eq!(effect, Effect::Deny);


