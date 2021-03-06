use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
}

impl MemberInfo {
    pub fn new(name: &str, access: &str, data_type: String) -> Self {
        MemberInfo {
            name: name.to_string(),
            access: access.to_string(),
            data_type: data_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub return_type: String,
}

impl MethodInfo {
    pub fn new(name: &str, access: &str, data_type: String) -> Self {
        MethodInfo {
            name: name.to_string(),
            access: access.to_string(),
            return_type: data_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub id: i32,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub methods: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(class_name: &str) -> Self {
        ClassInfo {
            name: class_name.to_string(),
            id: 0,
            parents: vec![],
            members: vec![],
            methods: vec![],
        }
    }
}
