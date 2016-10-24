use rustc::lint::{self, LintArray};
use syntax::ast;
use utils::span_lint;

/// **What it does:** Checks for items named `lintme`
///
/// **Why is this bad?** It looks suspiciously like a test case for this lint
///
/// **Known problems:** There might be a language in which "lintme" is a good name
///
/// **Example:**
/// ```rust
/// fn lintme(x: i32) -> i32 { x * 3 + 5 }
/// ```
declare_lint! {
    pub TEST_LINT,
    Warn,
    "item is named `lintme`"
}

pub struct Pass;

impl lint::LintPass for Pass {
    fn get_lints(&self) -> lint::LintArray {
        // list all lints that this `LintPass` can produce.
        // If a lint is emitted that is not listed here, rustc will abort the compilation
        lint_array!(TEST_LINT)
    }
}

impl lint::EarlyLintPass for Pass {
    // implement the `check_item` method to run custom code for every item
    fn check_item(&mut self, cx: &lint::EarlyContext, it: &ast::Item) {
        // The ast applies hygiene through `Ident` types.
        // We don't care about hygiene in this lint, and just want the textual name
        if it.ident.name.as_str() == "lintme" {
            // A helper from clippy that prints some information about the lint
            span_lint(cx, TEST_LINT, it.span, "item is named `lintme`");
        }
    }
}
