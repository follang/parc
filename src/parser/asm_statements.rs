fn __parse_asm_label<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Extension>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_asm_label0(__input, __state, __pos, env);
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

fn __parse_asm_label0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Extension> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_asm_label_keyword(__input, __state, __pos, env);
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
                                        let __seq_res = __parse_string_literal(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, s) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, { Extension::AsmLabel(s) }),
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

fn __parse_asm_label_keyword<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            __state.suppress_fail += 1;
            let res = {
                let __choice_res = {
                    __state.suppress_fail += 1;
                    let res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "asm");
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
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        __state.suppress_fail += 1;
                        let res = {
                            let __seq_res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "__asm");
                                match __seq_res {
                                    Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                        Matched(__newpos, _) => Matched(__newpos, ()),
                                        Failed => Matched(__pos, ()),
                                    },
                                    Failed => Failed,
                                }
                            };
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
                    }
                }
            };
            __state.suppress_fail -= 1;
            res
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                __state.mark_failure(__pos, "asm");
                Failed
            }
        }
    }
}

fn __parse_asm_statement<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Statement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_asm_statement0(__input, __state, __pos, env);
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
            Matched(__pos, s) => Matched(__pos, { Statement::Asm(s) }),
            Failed => Failed,
        }
    }
}

fn __parse_asm_statement0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<AsmStatement> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = {
                    let __choice_res = slice_eq(__input, __state, __pos, "asm");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __seq_res = slice_eq(__input, __state, __pos, "__asm");
                            match __seq_res {
                                Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                    Failed => Matched(__pos, ()),
                                },
                                Failed => Failed,
                            }
                        }
                    }
                };
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
                        let __seq_res = match __parse_type_qualifier(__input, __state, __pos, env) {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, q) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, "(");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_string_literal(__input, __state, __pos, env);
                                                        match __seq_res {
                                                            Matched(__pos, a) => {
                                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = match {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, ":");
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => {
                                                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                    match __seq_res {
                                                                                        Matched(__pos, _) => {
                                                                                            let __seq_res = __parse_asm_operand_list(__input, __state, __pos, env);
                                                                                            match __seq_res {
                                                                                                Matched(__pos, e) => {
                                                                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                    match __seq_res {
                                                                                                        Matched(__pos, _) => {
                                                                                                            let __seq_res = match {
                                                                                                                let __seq_res = slice_eq(__input, __state, __pos, ":");
                                                                                                                match __seq_res {
                                                                                                                    Matched(__pos, _) => {
                                                                                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                                        match __seq_res {
                                                                                                                            Matched(__pos, _) => {
                                                                                                                                let __seq_res = __parse_asm_operand_list(__input, __state, __pos, env);
                                                                                                                                match __seq_res {
                                                                                                                                    Matched(__pos, e) => {
                                                                                                                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                                                                                                                        match __seq_res {
                                                                                                                                            Matched(__pos, _) => {
                                                                                                                                                let __seq_res = match {
                                                                                                                                                    let __seq_res = slice_eq(__input, __state, __pos, ":");
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
                                                                                                                                                                                let __step_res = __parse_string_literal(__input, __state, __pos, env);
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
                                                                                                                                                                                    let __seq_res = match Matched(__pos, ()) {
                                                                                                                                                                                        Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                                                                                                                                        Failed => Matched(__pos, None),
                                                                                                                                                                                    };
                                                                                                                                                                                    match __seq_res {
                                                                                                                                                                                        Matched(__pos, t) => Matched(__pos, { (e, t.unwrap_or_default()) }),
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
                                                                                                                                                } {
                                                                                                                                                    Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                                                                                                    Failed => Matched(__pos, None),
                                                                                                                                                };
                                                                                                                                                match __seq_res {
                                                                                                                                                    Matched(__pos, t) => Matched(__pos, { (e, t.unwrap_or_default()) }),
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
                                                                                                            } {
                                                                                                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                                                                Failed => Matched(__pos, None),
                                                                                                            };
                                                                                                            match __seq_res {
                                                                                                                Matched(__pos, t) => Matched(__pos, { (e, t.unwrap_or_default()) }),
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
                                                                        } {
                                                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                                            Failed => Matched(__pos, None),
                                                                        };
                                                                        match __seq_res {
                                                                            Matched(__pos, o) => {
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
                                                                                                            Matched(__pos, _) => Matched(__pos, {
                                                                                                                if let Some((o, (i, (c, ())))) = o {
                                                                                                                    AsmStatement::GnuExtended(GnuExtendedAsmStatement { qualifier: q, template: a, outputs: o, inputs: i, clobbers: c })
                                                                                                                } else {
                                                                                                                    AsmStatement::GnuBasic(a)
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

fn __parse_asm_operand_list<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<GnuAsmOperand>>> {
    #![allow(non_snake_case, unused)]
    {
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
                            let __seq_res = __parse_asm_operand(__input, __state, __pos, env);
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
    }
}

fn __parse_asm_operand<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<GnuAsmOperand> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = match {
            let __seq_res = slice_eq(__input, __state, __pos, "[");
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
                                            let __seq_res = slice_eq(__input, __state, __pos, "]");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { i }),
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
        } {
            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
            Failed => Matched(__pos, None),
        };
        match __seq_res {
            Matched(__pos, i) => {
                let __seq_res = __parse_string_literal(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, s) => {
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
                                                                let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                                match __seq_res {
                                                                    Matched(__pos, _) => Matched(__pos, { GnuAsmOperand { symbolic_name: i, constraints: s, variable_name: e } }),
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

