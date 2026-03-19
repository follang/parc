fn __parse_attribute_specifier_list<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<Extension>>> {
    #![allow(non_snake_case, unused)]
    {
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
                    let __step_res = __parse_attribute_specifier(__input, __state, __pos, env);
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
            Matched(__pos, a) => Matched(__pos, { a.into_iter().flat_map(|v| v).collect() }),
            Failed => Failed,
        }
    }
}

fn __parse_attribute_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<Extension>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "__attribute");
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
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "((");
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
                                                                let __seq_res = __parse_attribute(__input, __state, __pos, env);
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
                                            Matched(__pos, a) => {
                                                let __seq_res = __parse__(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = slice_eq(__input, __state, __pos, "))");
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, { a }),
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

fn __parse_attribute<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Extension> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __seq_res = {
                    __state.suppress_fail += 1;
                    let __assert_res = __parse_clang_guard(__input, __state, __pos, env);
                    __state.suppress_fail -= 1;
                    match __assert_res {
                        Matched(_, __value) => Matched(__pos, __value),
                        Failed => Failed,
                    }
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_attr_availability(__input, __state, __pos, env);
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
            };
            match __seq_res {
                Matched(__pos, c) => Matched(__pos, { Extension::AvailabilityAttribute(c) }),
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
                            let __seq_res = __parse_attribute_name(__input, __state, __pos, env);
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
                            Matched(__pos, _) => {
                                let __seq_res = match __parse_attribute_parameters(__input, __state, __pos, env) {
                                    Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                    Failed => Matched(__pos, None),
                                };
                                match __seq_res {
                                    Matched(__pos, p) => Matched(__pos, { Extension::Attribute(Attribute { name: n, arguments: p.unwrap_or_default() }) }),
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

fn __parse_attribute_name<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<String> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = if __input.len() > __pos {
                        let (__ch, __next) = char_range_at(__input, __pos);
                        match __ch {
                            '_' | 'a'..='z' | 'A'..='Z' => Matched(__next, ()),
                            _ => __state.mark_failure(__pos, "[_a-zA-Z]"),
                        }
                    } else {
                        __state.mark_failure(__pos, "[_a-zA-Z]")
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let mut __repeat_pos = __pos;
                            loop {
                                let __pos = __repeat_pos;
                                let __step_res = if __input.len() > __pos {
                                    let (__ch, __next) = char_range_at(__input, __pos);
                                    match __ch {
                                        '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => Matched(__next, ()),
                                        _ => __state.mark_failure(__pos, "[_a-zA-Z0-9]"),
                                    }
                                } else {
                                    __state.mark_failure(__pos, "[_a-zA-Z0-9]")
                                };
                                match __step_res {
                                    Matched(__newpos, __value) => {
                                        __repeat_pos = __newpos;
                                    }
                                    Failed => {
                                        break;
                                    }
                                }
                            }
                            Matched(__repeat_pos, ())
                        }
                        Failed => Failed,
                    }
                };
                __state.suppress_fail -= 1;
                res
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => Matched(__pos, { String::from(n) }),
            Failed => Failed,
        }
    }
}

fn __parse_attribute_parameters<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
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

