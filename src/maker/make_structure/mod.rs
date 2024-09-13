use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MakeStruct {
    pub(crate) rules: HashMap<String, String>,
    pub(crate) vars: HashMap<String, String>,
    pub(crate) run: HashMap<String, Vec<String>>
}