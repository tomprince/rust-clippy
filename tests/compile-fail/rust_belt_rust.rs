#![feature(plugin)]
#![plugin(clippy)]
// only warn on AST lints, because they are run before the HIR lints and the HIR
// lints would never be run, if an AST lint caused an error
#![warn(test_lint)]
#![deny(test_lint2)]

fn lintme() {} //~ WARN item is named `lintme`

fn main() {
    let x: Box<std::fmt::Display> = Box::new(5);
    println!("{}", x); //~ ERROR usage of trait object
    lintme();
}
