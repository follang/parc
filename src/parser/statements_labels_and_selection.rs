fn __parse_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Statement>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_statement0(__input, __state, __pos, env);
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

fn __parse_statement0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = Matched(__pos, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse_labeled_statement(__input, __state, __pos, env);
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
                Matched(__pos, s) => Matched(__pos, { Statement::Labeled(s) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = Matched(__pos, {
                        env.enter_scope();
                    });
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = match __parse_compound_statement(__input, __state, __pos, env) {
                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                Failed => Matched(__pos, None),
                            };
                            match __seq_res {
                                Matched(__pos, e) => {
                                    match {
                                        env.leave_scope();
                                        e.ok_or("")
                                    } {
                                        Ok(res) => Matched(__pos, res),
                                        Err(expected) => {
                                            __state.mark_failure(__pos, expected);
                                            Failed
                                        }
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
                        let __choice_res = __parse_expression_statement(__input, __state, __pos, env);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = Matched(__pos, {
                                        env.enter_scope();
                                    });
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = match __parse_selection_statement(__input, __state, __pos, env) {
                                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                Failed => Matched(__pos, None),
                                            };
                                            match __seq_res {
                                                Matched(__pos, e) => {
                                                    match {
                                                        env.leave_scope();
                                                        e.ok_or("")
                                                    } {
                                                        Ok(res) => Matched(__pos, res),
                                                        Err(expected) => {
                                                            __state.mark_failure(__pos, expected);
                                                            Failed
                                                        }
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
                                            let __seq_res = Matched(__pos, {
                                                env.enter_scope();
                                            });
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = match __parse_iteration_statement(__input, __state, __pos, env) {
                                                        Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                        Failed => Matched(__pos, None),
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, e) => {
                                                            match {
                                                                env.leave_scope();
                                                                e.ok_or("")
                                                            } {
                                                                Ok(res) => Matched(__pos, res),
                                                                Err(expected) => {
                                                                    __state.mark_failure(__pos, expected);
                                                                    Failed
                                                                }
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
                                                let __choice_res = __parse_jump_statement(__input, __state, __pos, env);
                                                match __choice_res {
                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                    Failed => {
                                                        let __seq_res = {
                                                            __state.suppress_fail += 1;
                                                            let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                            __state.suppress_fail -= 1;
                                                            match __assert_res {
                                                                Matched(_, __value) => Matched(__pos, __value),
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse_asm_statement(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, e) => Matched(__pos, { e }),
                                                                    Failed => Failed,
                                                                }
                                                            }
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

fn __parse_labeled_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<LabeledStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_label(__input, __state, __pos, env);
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
            Matched(__pos, l) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, ":");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_statement(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, s) => Matched(__pos, { LabeledStatement { label: l, statement: s } }),
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

fn __parse_label<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Label> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_identifier(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, { Label::Identifier(i) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "case");
                            match __seq_res {
                                Matched(__pos, e) => {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let __assert_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                        };
                                        __state.suppress_fail -= 1;
                                        match __assert_res {
                                            Failed => Matched(__pos, ()),
                                            Matched(..) => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { e }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        };
                        __state.suppress_fail -= 1;
                        res
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse_constant_expression(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, a) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = match {
                                                        let __seq_res = {
                                                            __state.suppress_fail += 1;
                                                            let __assert_res = __parse_gnu_guard(__input, __state, __pos, env);
                                                            __state.suppress_fail -= 1;
                                                            match __assert_res {
                                                                Matched(_, __value) => Matched(__pos, __value),
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse_range_suffix(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, e) => Matched(__pos, { e }),
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    } {
                                                        Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                        Failed => Matched(__pos, None),
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, b) => Matched(__pos, {
                                                            match b {
                                                                Some(b) => {
                                                                    let span = Span::span(a.span.start, b.span.end);
                                                                    Label::CaseRange(Node::new(CaseRange { low: a, high: Box::new(b) }, span))
                                                                }
                                                                None => Label::Case(a),
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
                            __state.suppress_fail += 1;
                            let res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "default");
                                match __seq_res {
                                    Matched(__pos, e) => {
                                        let __seq_res = {
                                            __state.suppress_fail += 1;
                                            let __assert_res = if __input.len() > __pos {
                                                let (__ch, __next) = char_range_at(__input, __pos);
                                                match __ch {
                                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                                }
                                            } else {
                                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                            };
                                            __state.suppress_fail -= 1;
                                            match __assert_res {
                                                Failed => Matched(__pos, ()),
                                                Matched(..) => Failed,
                                            }
                                        };
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, { e }),
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                            __state.suppress_fail -= 1;
                            res
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { Label::Default }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_compound_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
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
                                                let __seq_res = __parse_block_item(__input, __state, __pos, env);
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
                            Matched(__pos, b) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, "}");
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, { Statement::Compound(b) }),
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

fn __parse_block_item<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<BlockItem> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_declaration(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, d) => Matched(__pos, { BlockItem::Declaration(d) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = __parse_static_assert(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, s) => Matched(__pos, { BlockItem::StaticAssert(s) }),
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
                                    let __seq_res = __parse_statement0(__input, __state, __pos, env);
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
                            Matched(__pos, s) => Matched(__pos, { BlockItem::Statement(s) }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_expression_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match __parse_expression(__input, __state, __pos, env) {
            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
            Failed => Matched(__pos, None),
        };
        match __seq_res {
            Matched(__pos, e) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, ";");
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { Statement::Expression(e) }),
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

fn __parse_selection_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = Matched(__pos, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse_if_statement(__input, __state, __pos, env);
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
                Matched(__pos, s) => Matched(__pos, { Statement::If(s) }),
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
                            let __seq_res = __parse_switch_statement(__input, __state, __pos, env);
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
                    Matched(__pos, s) => Matched(__pos, { Statement::Switch(s) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_if_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<IfStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "if");
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "(");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_expression(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, e) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = __parse_statement(__input, __state, __pos, env);
                                                                        match __seq_res {
                                                                            Matched(__pos, a) => {
                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = match __parse_else_statement(__input, __state, __pos, env) {
                                                                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                                            Failed => Matched(__pos, None),
                                                                                        };
                                                                                        match __seq_res {
                                                                                            Matched(__pos, b) => Matched(__pos, { IfStatement { condition: e, then_statement: a, else_statement: b } }),
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

fn __parse_else_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Statement>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "else");
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_statement(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, s) => Matched(__pos, { s }),
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

fn __parse_switch_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SwitchStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "switch");
                match __seq_res {
                    Matched(__pos, e) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { e }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "(");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_expression(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, e) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = __parse_statement(__input, __state, __pos, env);
                                                                        match __seq_res {
                                                                            Matched(__pos, s) => Matched(__pos, { SwitchStatement { expression: e, statement: s } }),
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

