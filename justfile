default:
    @just --list

check flags='':
    cargo check {{ flags }}
alias c := check

lint flags='':
    cargo clippy {{ flags }}
alias l := lint

run-example example:
    cargo run --example {{ example }}

test test_filter='':
    cargo nextest run -- {{ test_filter }}
alias t := test
