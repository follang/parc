fn __parse_cast_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_cast_expression0(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, e) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_cast_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = Matched(__pos, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse_cast_expression_inner(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, e) => {
                                let __seq_res = Matched(__pos, __pos);
                                match __seq_res {
                                    Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, c) => Matched(__pos, { Expression::Cast(Box::new(c)) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_unary_expression0(__input, __state, __pos, env),
        }
    }
}

fn __parse_cast_expression_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<CastExpression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "(");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_type_name(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, t) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_cast_expression(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, e) => Matched(__pos, { CastExpression { type_name: t, expression: e } }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_binary_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_binary_expression0(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, e) => Matched(__pos, { Box::new(e) }),
            Failed => Failed,
        }
    }
}

fn __parse_binary_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
    #![allow(non_snake_case, unused)]
    {
        fn __infix_parse<'input>(__min_prec: i32, __input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
            if let Matched(__pos, mut __infix_result) = __parse_binary_operand(__input, __state, __pos, env) {
                let mut __repeat_pos = __pos;
                loop {
                    let __pos = __repeat_pos;
                    if 0i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "||");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(1i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::LogicalOr, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 1i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "&&");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(2i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::LogicalAnd, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 2i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "|");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(3i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::BitwiseOr, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 3i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "^");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(4i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::BitwiseXor, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 4i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "&");
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            __state.suppress_fail += 1;
                                                            let __assert_res = slice_eq(__input, __state, __pos, "&");
                                                            __state.suppress_fail -= 1;
                                                            match __assert_res {
                                                                Failed => Matched(__pos, ()),
                                                                Matched(..) => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(5i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::BitwiseAnd, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 5i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "==");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(6i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Equals, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "!=");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(6i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::NotEquals, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 6i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "<");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(7i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Less, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, ">");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(7i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Greater, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "<=");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(7i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::LessOrEqual, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, ">=");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(7i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::GreaterOrEqual, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 7i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "<<");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(8i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::ShiftLeft, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, ">>");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(8i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::ShiftRight, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 8i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "+");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(9i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Plus, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "-");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(9i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Minus, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    if 9i32 >= __min_prec {
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "*");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(10i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Multiply, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "/");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(10i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Divide, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                        if let Matched(__pos, o) = {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, "%");
                                                match __seq_res {
                                                    Matched(__pos, e) => {
                                                        let __seq_res = Matched(__pos, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { n }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        } {
                            if let Matched(__pos, y) = __infix_parse(10i32, __input, __state, __pos, env) {
                                let x = __infix_result;
                                __infix_result = { infix(o, BinaryOperator::Modulo, x, y) };
                                __repeat_pos = __pos;
                                continue;
                            }
                        }
                    }
                    break;
                }
                Matched(__repeat_pos, __infix_result)
            } else {
                Failed
            }
        }
        __infix_parse(0, __input, __state, __pos, env)
    }
}

fn __parse_binary_operand<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_cast_expression0(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = Matched(__pos, __pos);
                        match __seq_res {
                            Matched(__pos, r) => Matched(__pos, { Node::new(e, Span::span(l, r)) }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

