use std::fs::File;
use std::io::{Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use crate::maker::make_structure::MakeStruct;

mod make_structure;
pub struct Maker {}

impl Maker {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) async fn make<P: AsRef<Path>>(&self, path: P) {
        let mut file = File::open(path).unwrap();
        let mut content = "".to_string();
        file.read_to_string(&mut content).unwrap();
        let mut mks = toml::from_str::<MakeStruct>(&content).unwrap();
        if mks.vars.is_some() {
            for (var, value) in mks.vars.clone().unwrap() {
                for action in mks.rules.values_mut() {
                    let mut var_ = String::from("{");
                    var_.push_str(var.as_str());
                    var_.push('}');
                    let buf = action.replace(&var_, &value);
                    *action = buf;
                }
            }
        }

        let rules = Arc::new(mks.rules.clone());
        let mut rule_stack = FuturesUnordered::new();
        for (_, run_trace) in mks.run.iter() {
            let run_trace = run_trace.clone();
            let rules_arc = rules.clone();
            rule_stack.push(tokio::spawn(async move{
                for field in run_trace {
                    if let Some(action) = rules_arc.get(&field) {
                        let mut commands = action.split(" ").collect::<Vec<&str>>();
                        let command = commands.remove(0);
                        let args = commands;
                        let mut action = Command::new(command);
                        action.args(args).stdout(Stdio::inherit());
                        if let Err(e) = action.spawn().unwrap().wait_with_output() {
                            panic!("{}", e)
                        }
                    }
                }
            }));
        }
        while rule_stack.next().await.is_some() {}
    }
}