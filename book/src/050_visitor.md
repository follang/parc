# Visitor Pattern

The `visit` module provides a trait-based visitor for walking the AST.

## Usage

Implement the `Visit` trait and override methods for the node types you care about:

```rust
use pac::visit::Visit;
use pac::ast::*;

struct FunctionCounter {
    count: usize,
}

impl<'ast> Visit<'ast> for FunctionCounter {
    fn visit_function_definition(
        &mut self,
        func: &'ast FunctionDefinition,
        span: &'ast Span,
    ) {
        self.count += 1;
        // call default to continue walking children
        self.visit_function_definition(func, span);
    }
}
```

The default implementation of each `visit_*` method walks into child nodes, so you only need to override the methods you're interested in.
