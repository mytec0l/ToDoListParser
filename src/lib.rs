use pest::Parser;
use anyhow::anyhow;
use anyhow::Result;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Complete,
    Incomplete,
}

pub fn status(input: &str) -> Result <TaskStatus> {
    let pair = Grammar::parse(Rule:: status, input)?.next().ok_or_else(|| anyhow!("no pair"))?;
    
    match pair.into_inner().next().unwrap().as_rule(){
        Rule::complete => Ok(TaskStatus::Complete),
        Rule::incomplete => Ok(TaskStatus::Incomplete),
        _ => panic!("Error")

    }
}