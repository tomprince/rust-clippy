use rustc::lint::{self, LintArray};
use syntax::ast;
use rustc::hir;
use rustc::ty::{TypeVariants, TypeAndMut};
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
    Allow,
    "item is named `lintme`"
}

/// **What it does:** Checks for uses of trait objects
///
/// **Why is this bad?** Dynamic dispatch might be undesirable in some scenarios
///
/// **Known problems:** None
///
/// **Example:**
/// ```rust
/// let x: Box<Display> = Box::new(5);
/// ```
declare_lint! {
    pub TEST_LINT2,
    Allow,
    "usage of trait objects"
}

pub struct AstPass;
pub struct HirPass;

impl lint::LintPass for AstPass {
    fn get_lints(&self) -> lint::LintArray {
        // list all lints that this `LintPass` can produce.
        // If a lint is emitted that is not listed here, rustc will abort the compilation
        lint_array!(TEST_LINT)
    }
}

impl lint::LintPass for HirPass {
    fn get_lints(&self) -> lint::LintArray {
        lint_array!(TEST_LINT2)
    }
}

impl lint::EarlyLintPass for AstPass {
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

impl lint::LateLintPass for HirPass {
    // implement the `check_expr` method to run custom code for every expression (and its sub-expressions)
    fn check_expr(&mut self, cx: &lint::LateContext, expr: &hir::Expr) {
        let expr_ty = cx.tcx.node_id_to_type(expr.id);
        match expr_ty.sty {
            TypeVariants::TyBox(ty) |
            TypeVariants::TyRef(_, TypeAndMut { ty, .. }) |
            TypeVariants::TyRawPtr(TypeAndMut { ty, .. }) => {
                // make sure custom unsized types are also detected
                let relevant_ty = cx.tcx.struct_tail(ty);
                if let TypeVariants::TyTrait(_) = relevant_ty.sty {
                    span_lint(cx, TEST_LINT2, expr.span, "usage of trait object");
                }
            },
            _ => {},
        }
    }
}
