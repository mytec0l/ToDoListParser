use anyhow::{anyhow, Result};
use pest::Parser;
use ToDoListParser::{Grammar, Rule};

#[test]
fn testComplete() -> anyhow::Result<()> {
    let input = "[x]";
    let pair = Grammar::parse(Rule::status, input)?.next().ok_or_else(|| anyhow!("Cannot find [x]"))?;
    assert_eq!(pair.as_rule(), Rule::status);
    assert_eq!(pair.as_span().end(), 3);
    let rule = pair.into_inner().next().unwrap().as_rule();
    assert_eq!(rule, Rule::complete);

    Ok(())
}