use std::collections::LinkedList;
use ic_cdk::export::Principal;
use crate::statement::RequestResource;

pub struct RequestResourceBuilder {
    data: LinkedList<String>,
}

impl RequestResourceBuilder {
    pub fn new(node: &str) -> RequestResourceBuilder {
        let mut data = LinkedList::new();
        data.push_front(node.to_string());
        RequestResourceBuilder { data }
    }

    pub fn add(mut self, node: &str) -> RequestResourceBuilder {
        self.data.push_front(node.to_string());
        return RequestResourceBuilder {
            data: self.data
        };
    }

    pub fn build(mut self) -> RequestResource {
        let mut output = RequestResource::Resource(self.data.pop_front().unwrap());

        while let Some(data) = self.data.pop_front() {
            let tmp = output;
            output = RequestResource::Nested {
                node: data,
                next: Some(Box::new(tmp)),
            }
        }
        return output;
    }
}

pub struct Request {
    action: String,
    resource: RequestResource,
    caller: Principal,
}

impl Request {
    pub fn new(action: String, resource: RequestResource, caller: Principal) -> Self {
        Request {
            action,
            resource,
            caller,
        }
    }
    pub fn action(&self) -> &String {
        &self.action
    }
    pub fn resource(&self) -> &RequestResource {
        &self.resource
    }
    pub fn caller(&self) -> Principal {
        self.caller
    }
}