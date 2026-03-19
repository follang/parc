fn __parse_translation_unit<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TranslationUnit> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match __parse_directive(__input, __state, __pos, env) {
            Matched(__newpos, _) => Matched(__newpos, ()),
            Failed => Matched(__pos, ()),
        };
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
                                                let __seq_res = __parse_external_declaration(__input, __state, __pos, env);
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
                            Matched(__pos, d) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => Matched(__pos, { TranslationUnit(d) }),
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

fn __parse_external_declaration<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ExternalDeclaration> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_external_declaration0(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, d) => {
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
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = vec![];
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, _) => slice_eq(__input, __state, __pos, ";"),
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
                        }
                        Failed => Failed,
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { d }),
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_external_declaration0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ExternalDeclaration> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_declaration(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, d) => Matched(__pos, { ExternalDeclaration::Declaration(d) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = __parse_static_assert(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, s) => Matched(__pos, { ExternalDeclaration::StaticAssert(s) }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, {
                                env.enter_scope();
                            });
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = match {
                                        let __seq_res = Matched(__pos, __pos);
                                        match __seq_res {
                                            Matched(__pos, l) => {
                                                let __seq_res = __parse_function_definition(__input, __state, __pos, env);
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
                                    } {
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
                        match __seq_res {
                            Matched(__pos, d) => Matched(__pos, { ExternalDeclaration::FunctionDefinition(d) }),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_function_definition<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FunctionDefinition> {
    #![allow(non_snake_case, unused)]
    {
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
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = slice_eq(__input, __state, __pos, "__extension__");
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
                        Matched(__pos, e) => Matched(__pos, { e }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        } {
            Matched(__newpos, _) => Matched(__newpos, ()),
            Failed => Matched(__pos, ()),
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_declaration_specifiers(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, a) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_declarator(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, b) => {
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
                                                                    let __step_res = __parse_declaration(__input, __state, __pos, env);
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
                                                            Matched(__pos, c) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = {
                                                                            let __seq_res = Matched(__pos, __pos);
                                                                            match __seq_res {
                                                                                Matched(__pos, l) => {
                                                                                    let __seq_res = __parse_compound_statement(__input, __state, __pos, env);
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
                                                                            Matched(__pos, d) => Matched(__pos, { FunctionDefinition { specifiers: a, declarator: b, declarations: c, statement: d } }),
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

fn __parse_gnu_guard<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    match {
        if env.extensions_gnu {
            Ok(())
        } else {
            Err("gnu extensions disabled")
        }
    } {
        Ok(res) => Matched(__pos, res),
        Err(expected) => {
            __state.mark_failure(__pos, expected);
            Failed
        }
    }
}

