use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeccompRule {
    apiVersion: String,
    kind: String,
    metadata: SeccompRuleMetadata,
    spec: SeccompRuleSpec
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeccompRuleMetadata {
    name: String,
    annotations: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeccompRuleSpec {
    defaultAction: String,
    architectures: Vec<String>,

    listenerPath: Option<String>,
    listenerMetadata: Option<String>,

    syscalls: Vec<SeccompSyscalls>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeccompSyscalls {
    action: String,
    names: Vec<String>
}

impl SeccompRule {
    pub fn create(name: String, defaultAction: String, architectures: Vec<String>) -> SeccompRule {
        let mut metadata = SeccompRuleMetadata {name: name, annotations: HashMap::new()};
        let mut spec = SeccompRuleSpec {defaultAction: defaultAction, architectures: architectures, listenerPath: None, listenerMetadata: None, syscalls: vec![]};
        let mut rule = SeccompRule {apiVersion: "security-profiles-operator.x-k8s.io/v1beta1".to_string(), kind: "SeccompProfile".to_string(), metadata: metadata, spec: spec};

        return rule;
    }

    pub fn setSpec(&mut self, spec: SeccompRuleSpec) {
        self.spec = spec;
    }

    pub fn getSpec(&mut self) -> &mut SeccompRuleSpec {
        return &mut self.spec;
    }

    pub fn getMetadata(&mut self) -> &mut SeccompRuleMetadata {
        return &mut self.metadata;
    }

    pub fn setMetadata(&mut self, metadata: SeccompRuleMetadata) {
        self.metadata = metadata;
    }
}

impl SeccompRuleMetadata {
    pub fn addAnnotation(&mut self, name: String, value: String) {
        self.annotations.insert(name, value);
    }

    pub fn removeAnnotation(&mut self, name: &String) {
        self.annotations.remove(name);
    }
}

impl SeccompRuleSpec {
    pub fn addRuleList(&mut self, action: String, syscalls: Vec<String>) {
        let syscall = SeccompSyscalls {action: action, names: syscalls};

        self.syscalls.push(syscall);
    }
}