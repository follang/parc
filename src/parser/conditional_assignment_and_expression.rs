fn __parse_conditional_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_conditional_expression0(__input, __state, __pos, env);
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

fn __parse_conditional_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_binary_expression0(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, a) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = match __parse_conditional_expressionT(__input, __state, __pos, env) {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, t) => Matched(__pos, {
                                if let Some((b, c)) = t {
                                    let span = Span::span(a.span.start, c.span.end);
                                    Expression::Conditional(Box::new(Node::new(ConditionalExpression { condition: Box::new(a), then_expression: b, else_expression: c }, span)))
                                } else {
                                    a.node
                                }
                            }),
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

fn __parse_conditional_expressionT<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<(Box<Node<Expression>>, Box<Node<Expression>>)> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "?");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_expression0(__input, __state, __pos, env);
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
                            Matched(__pos, a) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, ":");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = {
                                                            let __seq_res = Matched(__pos, __pos);
                                                            match __seq_res {
                                                                Matched(__pos, l) => {
                                                                    let __seq_res = __parse_conditional_expression0(__input, __state, __pos, env);
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
                                                            Matched(__pos, b) => Matched(__pos, { (Box::new(a), Box::new(b)) }),
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

fn __parse_assignment_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_assignment_expression0(__input, __state, __pos, env);
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

fn __parse_assignment_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = Matched(__pos, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse_assignment_expression_inner(__input, __state, __pos, env);
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
                Matched(__pos, n) => Matched(__pos, { Expression::BinaryOperator(Box::new(n)) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_conditional_expression0(__input, __state, __pos, env),
        }
    }
}

fn __parse_assignment_expression_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<BinaryOperatorExpression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_unary_expression(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, a) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_assignment_operator(__input, __state, __pos, env);
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
                            Matched(__pos, op) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, b) => Matched(__pos, { BinaryOperatorExpression { operator: op, lhs: a, rhs: b } }),
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

fn __parse_assignment_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<BinaryOperator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, "=");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { BinaryOperator::Assign }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "*=");
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignMultiply }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "/=");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignDivide }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "%=");
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignModulo }),
                                        Failed => Failed,
                                    }
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res = {
                                            let __seq_res = slice_eq(__input, __state, __pos, "+=");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignPlus }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __choice_res = {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "-=");
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignMinus }),
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __choice_res {
                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                    Failed => {
                                                        let __choice_res = {
                                                            let __seq_res = slice_eq(__input, __state, __pos, "<<=");
                                                            match __seq_res {
                                                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignShiftLeft }),
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __choice_res {
                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                            Failed => {
                                                                let __choice_res = {
                                                                    let __seq_res = slice_eq(__input, __state, __pos, ">>=");
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignShiftRight }),
                                                                        Failed => Failed,
                                                                    }
                                                                };
                                                                match __choice_res {
                                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                                    Failed => {
                                                                        let __choice_res = {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, "&=");
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignBitwiseAnd }),
                                                                                Failed => Failed,
                                                                            }
                                                                        };
                                                                        match __choice_res {
                                                                            Matched(__pos, __value) => Matched(__pos, __value),
                                                                            Failed => {
                                                                                let __choice_res = {
                                                                                    let __seq_res = slice_eq(__input, __state, __pos, "^=");
                                                                                    match __seq_res {
                                                                                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignBitwiseXor }),
                                                                                        Failed => Failed,
                                                                                    }
                                                                                };
                                                                                match __choice_res {
                                                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                                                    Failed => {
                                                                                        let __seq_res = slice_eq(__input, __state, __pos, "|=");
                                                                                        match __seq_res {
                                                                                            Matched(__pos, _) => Matched(__pos, { BinaryOperator::AssignBitwiseOr }),
                                                                                            Failed => Failed,
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn __parse_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_expression0(__input, __state, __pos, env);
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

fn __parse_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_assignment_expression0(__input, __state, __pos, env);
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
            Matched(__pos, e) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = vec![];
                                loop {
                                    let __pos = __repeat_pos;
                                    let __pos = if __repeat_value.len() > 0 {
                                        let __sep_res = __parse__(__input, __state, __pos, env);
                                        match __sep_res {
                                            Matched(__newpos, _) => __newpos,
                                            Failed => break,
                                        }
                                    } else {
                                        __pos
                                    };
                                    let __step_res = __parse_expressionT(__input, __state, __pos, env);
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, e) => Matched(__pos, { e }),
                                Failed => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, t) => Matched(__pos, {
                                if t.len() > 0 {
                                    let mut t = t;
                                    t.insert(0, e);
                                    Expression::Comma(Box::new(t))
                                } else {
                                    e.node
                                }
                            }),
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

fn __parse_expressionT<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, ",");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_assignment_expression0(__input, __state, __pos, env);
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
                            Matched(__pos, e) => Matched(__pos, { e }),
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

fn __parse_constant_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    __parse_conditional_expression(__input, __state, __pos, env)
}

fn __parse_constant_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    __parse_conditional_expression0(__input, __state, __pos, env)
}

