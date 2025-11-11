use anyhow::{Result, anyhow};
use pest::Parser;
use todo_list_parser::{Grammar, Rule};

#[test]
fn test_status_complete() -> Result<()> {
    let input = "[DONE]";
    let pair = Grammar::parse(Rule::status, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find [DONE]"))?;

    assert_eq!(pair.as_rule(), Rule::status);
    assert_eq!(pair.as_span().end(), 6);

    let inner_rule = pair.into_inner().next().unwrap().as_rule();
    assert_eq!(inner_rule, Rule::complete);

    Ok(())
}

#[test]
fn test_status_incomplete() -> Result<()> {
    let input = "[TODO]";
    let pair = Grammar::parse(Rule::status, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find [TODO]"))?;

    assert_eq!(pair.as_rule(), Rule::status);

    let inner_rule = pair.into_inner().next().unwrap().as_rule();
    assert_eq!(inner_rule, Rule::incomplete);

    Ok(())
}

#[test]
fn test_status_in_progress() -> Result<()> {
    let input = "[DOING]";
    let pair = Grammar::parse(Rule::status, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find [DOING]"))?;

    assert_eq!(pair.as_rule(), Rule::status);

    let inner_rule = pair.into_inner().next().unwrap().as_rule();
    assert_eq!(inner_rule, Rule::in_progress);

    Ok(())
}

#[test]
fn test_priority_one_star() -> Result<()> {
    let input = "*";
    let pair = Grammar::parse(Rule::priority, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find *"))?;

    assert_eq!(pair.as_rule(), Rule::priority);
    assert_eq!(pair.as_str(), "*");

    Ok(())
}

#[test]
fn test_priority_three_stars() -> Result<()> {
    let input = "***";
    let pair = Grammar::parse(Rule::priority, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find ***"))?;

    assert_eq!(pair.as_rule(), Rule::priority);
    assert_eq!(pair.as_str(), "***");

    Ok(())
}

#[test]
fn test_tag_simple() -> Result<()> {
    let input = "+work";
    let pair = Grammar::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find +work"))?;

    assert_eq!(pair.as_rule(), Rule::tag);
    assert_eq!(pair.as_str(), "+work");

    Ok(())
}

#[test]
fn test_tag_ukr() -> Result<()> {
    let input = "+документація";
    let pair = Grammar::parse(Rule::tag, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find +документація"))?;

    assert_eq!(pair.as_rule(), Rule::tag);
    assert_eq!(pair.as_str(), "+документація");

    Ok(())
}

#[test]
fn test_due_date() -> Result<()> {
    let input = "@2025-11-15";
    let pair = Grammar::parse(Rule::due_date, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find @2025-11-15"))?;

    assert_eq!(pair.as_rule(), Rule::due_date);
    assert_eq!(pair.as_str(), "@2025-11-15");

    Ok(())
}

#[test]
fn test_due_date2() -> Result<()> {
    let input = "@2026-12-31";
    let pair = Grammar::parse(Rule::due_date, input)?
        .next()
        .ok_or_else(|| anyhow!("Cannot find @2026-12-31"))?;

    assert_eq!(pair.as_rule(), Rule::due_date);
    assert_eq!(pair.as_str(), "@2026-12-31");

    Ok(())
}

#[test]
fn test_parse_simple_task() {
    let input = "[TODO] Simple task\n";
    let tasks = todo_list_parser::parse_file(input).unwrap();

    assert_eq!(tasks.len(), 1);
}

#[test]
fn test_parse_task_with_priority() {
    let input = "** [DOING] Important task\n";
    let tasks = todo_list_parser::parse_file(input).unwrap();

    assert_eq!(tasks.len(), 1);
}

#[test]
fn test_parse_multiple_tasks() {
    let input = "[TODO] First task\n[DOING] Second task\n";
    let tasks = todo_list_parser::parse_file(input).unwrap();

    assert_eq!(tasks.len(), 2);
}
