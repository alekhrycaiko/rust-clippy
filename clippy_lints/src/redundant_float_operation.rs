use crate::consts::{constant, Constant};
use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_hir::ExprKind::{MethodCall};
use rustc_hir::{Expr};
use crate::utils::{span_lint};

declare_clippy_lint! {
    /// **What it does:** Checks whether a constant float is making an unnecessary method call.
    ///
    /// **Why is this bad?** The method call can be avoided since the operation no-ops.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    /// ```rust
    /// 1f32.round();
    /// 1f64.ceil();
    /// 1f64.floor();
    /// ```
    pub REDUNDANT_FLOAT_OPERATION,
    complexity,
    "a known const float making a method call that no-ops"
}

declare_lint_pass!(RedundantFloatOperation => [REDUNDANT_FLOAT_OPERATION]);

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for RedundantFloatOperation {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr<'_>) {
        match &expr.kind {
            MethodCall(method_path, _, _args) => {
                let method_name = method_path.ident.name.as_str();
                if method_name == "round" || method_name == "float" || method_name == "ceil" {
                    match constant(cx, cx.tables, expr) {
                        Some((Constant::F32(f), _)) => {
                            if f % 0_f32 == 0_f32 {
                                span_lint(
                                    cx,
                                    REDUNDANT_FLOAT_OPERATION,
                                    expr.span,
                                    &format!("{} called redundantly", method_name)
                                );
                            }
                        },
                        Some((Constant::F64(f), _)) => {
                            if f % 0_f64 == 0_f64 {
                                span_lint(
                                    cx,
                                    REDUNDANT_FLOAT_OPERATION,
                                    expr.span,
                                    &format!("{} called redundantly", method_name)
                                );
                            }
                        }
                        _ => {}


                    }

                }
            },
            _ => {}
        }
    }
}
