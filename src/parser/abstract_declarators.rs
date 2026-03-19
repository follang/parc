fn __parse_abstract_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Declarator>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_abstract_declarator0(__input, __state, __pos, env);
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

fn __parse_abstract_declarator0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Declarator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
                        let __step_res = __parse_pointer(__input, __state, __pos, env);
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
                Matched(__pos, p) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let __seq_res = Matched(__pos, __pos);
                                match __seq_res {
                                    Matched(__pos, l) => {
                                        let __seq_res = __parse_direct_abstract_declarator(__input, __state, __pos, env);
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
                                Matched(__pos, k) => {
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
                                                        let __step_res = __parse_derived_abstract_declarator(__input, __state, __pos, env);
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
                                                Matched(__pos, d) => Matched(__pos, { Declarator { kind: k, derived: concat(p, d), extensions: Vec::new() } }),
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
                                let __step_res = __parse_pointer(__input, __state, __pos, env);
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
                        Matched(__pos, p) => {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, k) => {
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
                                                        let __step_res = __parse_derived_abstract_declarator(__input, __state, __pos, env);
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
                                                Matched(__pos, d) => Matched(__pos, { Declarator { kind: Node::new(DeclaratorKind::Abstract, Span::span(k, k)), derived: concat(p, d), extensions: Vec::new() } }),
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
                                    let __step_res = __parse_pointer(__input, __state, __pos, env);
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
                            Matched(__pos, p) => {
                                let __seq_res = Matched(__pos, __pos);
                                match __seq_res {
                                    Matched(__pos, k) => Matched(__pos, { Declarator { kind: Node::new(DeclaratorKind::Abstract, Span::span(k, k)), derived: p, extensions: Vec::new() } }),
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

fn __parse_direct_abstract_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<DeclaratorKind> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "(");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_abstract_declarator(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, d) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = slice_eq(__input, __state, __pos, ")");
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, { DeclaratorKind::Declarator(Box::new(d)) }),
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

fn __parse_derived_abstract_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<DerivedDeclarator>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_derived_abstract_declarator0(__input, __state, __pos, env);
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

fn __parse_derived_abstract_declarator0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<DerivedDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
                                        let __seq_res = __parse_abstract_array_declarator(__input, __state, __pos, env);
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
                                Matched(__pos, a) => Matched(__pos, { DerivedDeclarator::Array(a) }),
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
                                            let __seq_res = __parse_abstract_function_declarator(__input, __state, __pos, env);
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
                                    Matched(__pos, d) => {
                                        let __seq_res = __parse__(__input, __state, __pos, env);
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = slice_eq(__input, __state, __pos, ")");
                                                match __seq_res {
                                                    Matched(__pos, _) => Matched(__pos, { DerivedDeclarator::Function(d) }),
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

fn __parse_abstract_array_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ArrayDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
                        let __step_res = __parse_type_qualifier(__input, __state, __pos, env);
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
                Matched(__pos, q) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, "]");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { ArrayDeclarator { qualifiers: q, size: ArraySize::Unknown } }),
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
                                let __step_res = __parse_type_qualifier(__input, __state, __pos, env);
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
                        Matched(__pos, q) => {
                            let __seq_res = __parse__(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, e) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "]");
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { ArrayDeclarator { qualifiers: q, size: ArraySize::VariableExpression(e) } }),
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
                                    let __seq_res = slice_eq(__input, __state, __pos, "static");
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
                                                        let __step_res = __parse_type_qualifier(__input, __state, __pos, env);
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
                                                Matched(__pos, q) => {
                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                                            match __seq_res {
                                                                Matched(__pos, e) => {
                                                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res = slice_eq(__input, __state, __pos, "]");
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => Matched(__pos, { ArrayDeclarator { qualifiers: q, size: ArraySize::StaticExpression(e) } }),
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
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
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
                                                let __step_res = __parse_type_qualifier(__input, __state, __pos, env);
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
                                        Matched(__pos, q) => {
                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res = {
                                                        __state.suppress_fail += 1;
                                                        let res = {
                                                            let __seq_res = slice_eq(__input, __state, __pos, "static");
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
                                                                    let __seq_res = __parse_assignment_expression(__input, __state, __pos, env);
                                                                    match __seq_res {
                                                                        Matched(__pos, e) => {
                                                                            let __seq_res = __parse__(__input, __state, __pos, env);
                                                                            match __seq_res {
                                                                                Matched(__pos, _) => {
                                                                                    let __seq_res = slice_eq(__input, __state, __pos, "]");
                                                                                    match __seq_res {
                                                                                        Matched(__pos, _) => Matched(__pos, { ArrayDeclarator { qualifiers: q, size: ArraySize::StaticExpression(e) } }),
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
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __seq_res = slice_eq(__input, __state, __pos, "*");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, "]");
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, { ArrayDeclarator { qualifiers: Vec::new(), size: ArraySize::VariableUnknown } }),
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
    }
}

fn __parse_abstract_function_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FunctionDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
                        let __step_res = __parse_parameter_declaration(__input, __state, __pos, env);
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
                Matched(__pos, p) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_ellipsis(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, e) => Matched(__pos, { FunctionDeclarator { parameters: p, ellipsis: e } }),
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
            Failed => Matched(__pos, { FunctionDeclarator { parameters: Vec::new(), ellipsis: Ellipsis::None } }),
        }
    }
}

