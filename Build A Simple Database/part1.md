# Sqlite

![sqlite architecture](https://cstack.github.io/db_tutorial/assets/images/arch1.gif)
[sqlite – how it works](https://www.sqlite.org/zipvfs/doc/trunk/www/howitworks.wiki)

A query goes through a chain of components in order to retrieve or modify data.

## Front-end

`Tokenizer` > `Parser` > `Code Generator`

The input for the front-end is a SQL query and the output is sqlite virtual machine bytecode (a binary that can operate on the database)

## Back-end

`Virtual Machine` > `B-tree` > `Pager` > `OS Interface`

The **VM** takes the bytecode output from the front-end as instructions to
perform operations on the table(s) or index(es) which are stored in a **B-tree**.

Each **B-tree** has many nodes which are each one page in length.  The **B-tree*
is the interface between the **VM** and the **Pager**.

The **pager** receives commands to read/write pages of data.  It keeps track of
read/write of offsets in the database file and keeping a cache of
recently-accessed pages in memory.

The **OS Interface** is the layer that differs by the OS sqlite was compiled
for.

## Making a Simple REPL

I decided to do this in Rust, so first thing was first: setuping up a working
Rust environment

### 1. Setting up Rust

1. `curl https://sh.rustup.rs -sSf | sh`
2. `source $HOME/.cargo/env`
3. `echo "export PATH="$HOME/.cargo/bin:$PATH" >> ~/.zshrc.local`
4. `rustup update`
5. `rustc --version && cargo --version`

Easy!

### 2. Starting a new Rust project

I decided to call this sqliter, since it will most likely be the litest
version of sqlite.

`cargo new sqliter && cd sqliter`

Just for fun, let's make sure it's working:

`cargo run`

Bingo!

### 3.  Making a Simple REPL

Leared a few Rust fundamentals:

- Macros vs. functions
- Using packages with `use` keyword
- Working with mutable data structures with `mut` keyword
- Reading from `io::stdin()` and handling `Result` with `expect` (TLDR; allows you to quickly break on error but return the result otherwise)
- Using simple `loop` keyword (Ruby equivalent)
- Using a switch with `match` and handling default cases with `_`
- Flushing stdout to make a command prompt (`io::stdout().flush()`)

All-in-all this was a great exercise to get familiar with Rust.

```rust
use std::io::{self, Write};

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn main() {
    let mut command = String::new();

    println!("SQLiter version 0.1.0");

    loop {
        print_prompt();

        io::stdin().read_line(&mut command).expect("Failed to read line");

        match command.trim() {
            ".exit" => break,
            _ => print!("Unrecognized command: \'{}\' ", command),
        }
    }
}
```
