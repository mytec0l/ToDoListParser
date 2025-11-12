# Todo List Parser

A lightweight, command-line tool written in Rust for parsing, organizing, and sorting task lists from plain text files. This parser provides comprehensive task management capabilities including priority levels, status tracking, and date-based organization.

[Crates.io](https://crates.io/crates/todo_list_parser)

## Technical Description

### Parsing Process

The Todo List Parser uses the **Pest parser generator** to read and parse task files. The parser reads each line and extracts:

- **Priority** (optional): Represented by `*`, `**`, or `***`
- **Status** (required): One of `[TODO]`, `[DOING]`, or `[DONE]`
- **Description**: Text content that may include tags (`+tag`), due dates (`@YYYY-MM-DD`), and start dates (`^YYYY-MM-DD`)

Each line is validated against the grammar rules and converted into a `Task` structure containing priority, status, and description components.

### Results Processing

Once parsed, tasks are stored as `Task` objects in memory. The results can be:

- Displayed with formatted priority and status indicators
- Sorted by priority, status, start date, or due date
- Organized according to task properties

## Grammar Specification

```pest
WHITESPACE = _{ " " | "\t" }
NEWLINE = { "\r\n" | "\n" | "\r" }

// Status indicators (case-insensitive)
complete = { "[" ~ ("D"|"d") ~ ("O"|"o") ~ ("N"|"n") ~ ("E"|"e") ~ "]" }
incomplete = { "[" ~ ("T"|"t") ~ ("O"|"o") ~ ("D"|"d") ~ ("O"|"o") ~ "]" }
in_progress = { "[" ~ ("D"|"d") ~ ("O"|"o") ~ ("I"|"i") ~ ("N"|"n") ~ ("G"|"g") ~ "]" }

// Priority levels
priority = { "***" | "**" | "*" }

// Date components (YYYY-MM-DD format)
year = { ASCII_DIGIT{4} }
month = { ASCII_DIGIT{2} }
day = { ASCII_DIGIT{2} }

// Date indicators
due_date = { "@" ~ year ~ "-" ~ month ~ "-" ~ day }
start_date = { "^" ~ year ~ "-" ~ month ~ "-" ~ day }

// Tags and text
tag = { "+" ~ (LETTER | NUMBER | "_" | "-")+ }
text = @{ (!("+" | "@" | "^" | NEWLINE) ~ ANY)+ }

// Task structure
description_part = { due_date | start_date | tag | text }
task = { priority? ~ status ~ description }

// File structure
file = { SOI ~ (task ~ NEWLINE | NEWLINE)* ~ task? ~ NEWLINE? ~ EOI }
```

### Grammar Components

| Component | Format | Example | Required |
|-----------|--------|---------|----------|
| Priority | `*`, `**`, `***` | `***` (P1) | ✗ |
| Status | `[TODO]`, `[DOING]`, `[DONE]` | `[DONE]` | ✓ |
| Text | Plain text | `Write documentation` | ✓ |
| Tag | `+tag_name` | `+rust` | ✗ |
| Due Date | `@YYYY-MM-DD` | `@2025-12-31` | ✗ |
| Start Date | `^YYYY-MM-DD` | `^2025-11-10` | ✗ |

## Usage

```bash
# Parse tasks without sorting
cargo run -- parse test.txt

# Sort by priority
cargo run -- parse test.txt --sort-by-priority

# Sort by status
cargo run -- parse test.txt --sort-by-status

# Sort by start date
cargo run -- parse test.txt --sort-by-start

# Sort by due date
cargo run -- parse test.txt --sort-by-due

# View credits
cargo run -- credits

# View help
cargo run -- help
```