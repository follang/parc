fn __parse_postfix_expression<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_postfix_expression0(__input, __state, __pos, env);
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

fn __parse_postfix_expression0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    if let Some(entry) = __state.postfix_expression0_cache.get(&__pos) {
        return entry.clone();
    }
    let __rule_result = {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_postfix_expression1(__input, __state, __pos, env);
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
                                    let __step_res = {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = __parse_postfix_expressionT(__input, __state, __pos, env);
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
                            Matched(__pos, t) => Matched(__pos, { apply_ops(t, e).node }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    };
    __state.postfix_expression0_cache.insert(__pos, __rule_result.clone());
    __rule_result
}

fn __parse_postfix_expression1<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = __parse_compound_literal(__input, __state, __pos, env);
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_primary_expression0(__input, __state, __pos, env),
        }
    }
}

fn __parse_postfix_expressionT<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Operation> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = __parse_index_operator(__input, __state, __pos, env);
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "(");
                    match __seq_res {
                        Matched(__pos, _) => {
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
                                                    let __sep_res = {
                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                                match __seq_res {
                                                                    Matched(__pos, _) => __parse__(__input, __state, __pos, env),
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    };
                                                    match __sep_res {
                                                        Matched(__newpos, _) => __newpos,
                                                        Failed => break,
                                                    }
                                                } else {
                                                    __pos
                                                };
                                                let __step_res = {
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
                                        Matched(__pos, e) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { Operation::Call(e) }),
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
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                let __seq_res = Matched(__pos, __pos);
                                match __seq_res {
                                    Matched(__pos, l) => {
                                        let __seq_res = __parse_member_operator(__input, __state, __pos, env);
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
                                Matched(__pos, o) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse_identifier(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, i) => Matched(__pos, { Operation::Member(o, i) }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __seq_res = {
                                    let __seq_res = Matched(__pos, __pos);
                                    match __seq_res {
                                        Matched(__pos, l) => {
                                            let __seq_res = __parse_postfix_operator(__input, __state, __pos, env);
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
                                    Matched(__pos, o) => Matched(__pos, { Operation::Unary(o) }),
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

fn __parse_index_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Operation> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_index_operator0(__input, __state, __pos, env);
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
            Matched(__pos, i) => Matched(__pos, { Operation::Binary(Node::new(BinaryOperator::Index, i.span), i.node) }),
            Failed => Failed,
        }
    }
}

fn __parse_index_operator0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Expression>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "[");
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
                            Matched(__pos, e) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, "]");
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, { e }),
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

fn __parse_member_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<MemberOperator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, ".");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { MemberOperator::Direct }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = slice_eq(__input, __state, __pos, "->");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { MemberOperator::Indirect }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_postfix_operator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<UnaryOperator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, "++");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { UnaryOperator::PostIncrement }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = slice_eq(__input, __state, __pos, "--");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { UnaryOperator::PostDecrement }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_compound_literal<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Expression> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_compound_literal_inner(__input, __state, __pos, env);
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
            Matched(__pos, n) => Matched(__pos, { Expression::CompoundLiteral(Box::new(n)) }),
            Failed => Failed,
        }
    }
}

fn __parse_compound_literal_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<CompoundLiteral> {
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
                                                        let __seq_res = slice_eq(__input, __state, __pos, "{");
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
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
                                                                                        let __sep_res = {
                                                                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                            match __seq_res {
                                                                                                Matched(__pos, _) => {
                                                                                                    let __seq_res = slice_eq(__input, __state, __pos, ",");
                                                                                                    match __seq_res {
                                                                                                        Matched(__pos, _) => __parse__(__input, __state, __pos, env),
                                                                                                        Failed => Failed,
                                                                                                    }
                                                                                                }
                                                                                                Failed => Failed,
                                                                                            }
                                                                                        };
                                                                                        match __sep_res {
                                                                                            Matched(__newpos, _) => __newpos,
                                                                                            Failed => break,
                                                                                        }
                                                                                    } else {
                                                                                        __pos
                                                                                    };
                                                                                    let __step_res = {
                                                                                        let __seq_res = Matched(__pos, __pos);
                                                                                        match __seq_res {
                                                                                            Matched(__pos, l) => {
                                                                                                let __seq_res = __parse_initializer_list_item(__input, __state, __pos, env);
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
                                                                                if __repeat_value.len() >= 1 {
                                                                                    Matched(__repeat_pos, __repeat_value)
                                                                                } else {
                                                                                    Failed
                                                                                }
                                                                            };
                                                                            match __seq_res {
                                                                                Matched(__pos, e) => Matched(__pos, { e }),
                                                                                Failed => Failed,
                                                                            }
                                                                        };
                                                                        match __seq_res {
                                                                            Matched(__pos, i) => {
                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = match slice_eq(__input, __state, __pos, ",") {
                                                                                            Matched(__newpos, _) => Matched(__newpos, ()),
                                                                                            Failed => Matched(__pos, ()),
                                                                                        };
                                                                                        match __seq_res {
                                                                                            Matched(__pos, _) => {
                                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                match __seq_res {
                                                                                                    Matched(__pos, _) => {
                                                                                                        let __seq_res = slice_eq(__input, __state, __pos, "}");
                                                                                                        match __seq_res {
                                                                                                            Matched(__pos, _) => Matched(__pos, { CompoundLiteral { type_name: t, initializer_list: i } }),
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

