#![feature(plugin)]
#![plugin(clippy)]
#![deny(test_lint)]

fn lintme() {} //~ ERROR: item is named `lintme`

fn main() {
}
