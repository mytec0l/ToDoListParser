# Todo List Parser 

    A simple tool written in Rust for parsing and sorting .txt files containing lists of tasks.

    The parser (pest) parses the file, looking for tasks. Each task consists of:

        *priority (Optional): Priority (*, **, or ***).

        *status (Required): Status ([TODO], [DOING], or [DONE]).

        *description (Required): Description, which can contain:

        *text: Plain text.

        *tag: Tags (e.g., +rust).

        *due_date: Due date (e.g., @2025-12-31).

        *start_date: Start date (e.g., ^2025-11-10).

## CLI usage

    This tool uses clap to parse command line arguments.

Parsing and Sorting
    The main command is parse. It takes a filename and optional sorting flags:

    ```
    cargo run -- parse test.txt
    cargo run -- parse test.txt --sort-by-priority
    cargo run -- parse test.txt --sort-by-status
    cargo run -- parse test.txt --sort-by-start
    cargo run -- parse test.txt --sort-by-due
    ```

    You have also another commands:
    ```
    cargo test
    cargo run -- help
    cargo run -- credits
    ...
    ```

### Input.txt and Outpt

    ```
    *** [DONE] Write Reamde @2025-11-15 ^2025-11-01
    * [TODO] Drink water @2025-11-10 ^2025-11-05
    ** [DOING] Go sleep @2025-11-20 ^2025-11-03
    [TODO] Do rustlinks @2025-11-12
    *** [TODO] Exorcism 99 @2025-11-08 ^2025-11-02
    * [DONE] Exorcism 60 @2025-11-25 ^2025-11-04
    [DOING] Have a dinner ^2025-11-06
    ** [TODO] Wake up
    ```

    ```
    There is 8  tasks | sorting by just list
    (***) [DONE] Write Reamde @2025-11-15 ^2025-11-01
    (*) [TODO] Drink water @2025-11-10 ^2025-11-05
    (**) [DOING] Go sleep @2025-11-20 ^2025-11-03
    [TODO] Do rustlinks @2025-11-12
    (***) [TODO] Exorcism 99 @2025-11-08 ^2025-11-02
    (*) [DONE] Exorcism 60 @2025-11-25 ^2025-11-04
    [DOING] Have a dinner ^2025-11-06
    (**) [TODO] Wake up
    ```