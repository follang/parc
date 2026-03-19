fn __parse_iteration_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = Matched(__pos, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse_while_statement(__input, __state, __pos, env);
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
                Matched(__pos, s) => Matched(__pos, { Statement::While(s) }),
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
                                let __seq_res = __parse_do_while_statement(__input, __state, __pos, env);
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
                        Matched(__pos, s) => Matched(__pos, { Statement::DoWhile(s) }),
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
                                    let __seq_res = __parse_for_statement(__input, __state, __pos, env);
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
                            Matched(__pos, s) => Matched(__pos, { Statement::For(s) }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_while_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<WhileStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "while");
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
                                                                            Matched(__pos, s) => Matched(__pos, { WhileStatement { expression: e, statement: s } }),
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

fn __parse_do_while_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<DoWhileStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "do");
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
                            Matched(__pos, s) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = {
                                            __state.suppress_fail += 1;
                                            let res = {
                                                let __seq_res = slice_eq(__input, __state, __pos, "while");
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
                                                                                                        let __seq_res = slice_eq(__input, __state, __pos, ";");
                                                                                                        match __seq_res {
                                                                                                            Matched(__pos, _) => Matched(__pos, { DoWhileStatement { statement: s, expression: e } }),
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

fn __parse_for_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ForStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = slice_eq(__input, __state, __pos, "for");
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
                                        let __seq_res = {
                                            let __seq_res = Matched(__pos, __pos);
                                            match __seq_res {
                                                Matched(__pos, l) => {
                                                    let __seq_res = __parse_for_initializer(__input, __state, __pos, env);
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
                                                        let __seq_res = match __parse_expression(__input, __state, __pos, env) {
                                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                            Failed => Matched(__pos, None),
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, b) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = slice_eq(__input, __state, __pos, ";");
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => {
                                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                match __seq_res {
                                                                                    Matched(__pos, _) => {
                                                                                        let __seq_res = match __parse_expression(__input, __state, __pos, env) {
                                                                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                                            Failed => Matched(__pos, None),
                                                                                        };
                                                                                        match __seq_res {
                                                                                            Matched(__pos, c) => {
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
                                                                                                                            Matched(__pos, s) => Matched(__pos, { ForStatement { initializer: a, condition: b, step: c, statement: s } }),
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
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_for_initializer<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ForInitializer> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_expression(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, e) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ";");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { ForInitializer::Expression(e) }),
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
                    let __seq_res = __parse_declaration(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, d) => Matched(__pos, { ForInitializer::Declaration(d) }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = __parse_static_assert(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, s) => Matched(__pos, { ForInitializer::StaticAssert(s) }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __seq_res = slice_eq(__input, __state, __pos, ";");
                                match __seq_res {
                                    Matched(__pos, _) => Matched(__pos, { ForInitializer::Empty }),
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

fn __parse_jump_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "goto");
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
                            let __seq_res = __parse_identifier(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, i) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, ";");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { Statement::Goto(i) }),
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
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "continue");
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
                                    let __seq_res = slice_eq(__input, __state, __pos, ";");
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { Statement::Continue }),
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
                                __state.suppress_fail += 1;
                                let res = {
                                    let __seq_res = slice_eq(__input, __state, __pos, "break");
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
                                            let __seq_res = slice_eq(__input, __state, __pos, ";");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { Statement::Break }),
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
                                        let __seq_res = slice_eq(__input, __state, __pos, "return");
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
                                                                    Matched(__pos, _) => Matched(__pos, { Statement::Return(e) }),
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
                    }
                }
            }
        }
    }
}

