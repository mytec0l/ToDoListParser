use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Priority {
    P1,
    P2,
    P3,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DescriptionPart {
    Text(String),
    Tag(String),
    DueDate(String),
    StartDate(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    pub priority: Option<Priority>,
    pub status: Status,
    pub description: Vec<DescriptionPart>,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse file: {0}")]
    PestError(Box<dyn StdError + Send + Sync>),

    #[error("No tasks found in file")]
    EmptyFile,

    #[error("Task missing required status")]
    MissingStatus,
}

fn build_priority(pair: Pair<Rule>) -> Priority {
    match pair.as_str() {
        "*" => Priority::P3,
        "**" => Priority::P2,
        "***" => Priority::P1,
        _ => unreachable!(),
    }
}

fn build_status(pair: Pair<Rule>) -> Status {
    match pair.into_inner().next().unwrap().as_rule() {
        Rule::complete => Status::Done,
        Rule::incomplete => Status::Todo,
        Rule::in_progress => Status::Doing,
        _ => unreachable!(),
    }
}

fn build_description(pair: Pair<Rule>) -> Vec<DescriptionPart> {
    let mut parts = Vec::new();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::description_part => {
                let inner = part.into_inner().next().unwrap();
                let part_enum = match inner.as_rule() {
                    Rule::text => DescriptionPart::Text(inner.as_str().to_string()),
                    Rule::tag => DescriptionPart::Tag(inner.as_str().to_string()),
                    Rule::due_date => DescriptionPart::DueDate(inner.as_str().to_string()),
                    Rule::start_date => DescriptionPart::StartDate(inner.as_str().to_string()),
                    _ => unreachable!("Error rule in description {:?}", inner.as_rule()),
                };
                parts.push(part_enum);
            }
            _ => unreachable!(),
        }
    }
    parts
}

fn build_task(pair: Pair<Rule>) -> Result<Task, ParseError> {
    let mut priority = None;
    let mut status = None;
    let mut description = Vec::new();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::priority => priority = Some(build_priority(part)),
            Rule::status => status = Some(build_status(part)),
            Rule::description => description = build_description(part),
            _ => unreachable!("Error rule in the task {:?}", part.as_rule()),
        }
    }

    Ok(Task {
        priority,
        status: status.ok_or(ParseError::MissingStatus)?,
        description,
    })
}

pub fn parse_file(file_content: &str) -> Result<Vec<Task>, ParseError> {
    let file_pair = Grammar::parse(Rule::file, file_content)
        .map_err(|e| ParseError::PestError(Box::new(e)))?
        .next()
        .ok_or(ParseError::EmptyFile)?;

    let mut tasks = Vec::new();

    for pair in file_pair.into_inner() {
        match pair.as_rule() {
            Rule::task => {
                tasks.push(build_task(pair)?);
            }
            Rule::empty_line | Rule::NEWLINE | Rule::EOI => {}
            _ => unreachable!("Error rule in file {:?}", pair.as_rule()),
        }
    }

    Ok(tasks)
}
