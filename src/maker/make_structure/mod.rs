use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MakeStruct {
    pub(crate) rules: HashMap<String, String>,
    pub(crate) vars: Option<HashMap<String, String>>,
    pub(crate) run: Option<HashMap<String, Vec<String>>>,
    pub(crate) order: Option<Order>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub(crate) order: Vec<Vec<String>>
}