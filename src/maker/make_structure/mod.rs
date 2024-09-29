use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MakeStruct {
    pub(crate) rules: HashMap<String, String>,
    pub(crate) vars: Option<HashMap<String, String>>,
    pub(crate) run: Option<HashMap<String, Vec<String>>>
}