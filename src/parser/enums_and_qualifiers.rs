fn __parse_enum_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<EnumType> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = slice_eq(__input, __state, __pos, "enum");
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
                            let __seq_res = match __parse_identifier(__input, __state, __pos, env) {
                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                Failed => Matched(__pos, None),
                            };
                            match __seq_res {
                                Matched(__pos, i) => {
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
                                                                                    let __seq_res = __parse_enumerator(__input, __state, __pos, env);
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
                                                                Matched(__pos, e) => {
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
                                                                                                Matched(__pos, _) => Matched(__pos, { EnumType { identifier: i, enumerators: e } }),
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
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = {
                    __state.suppress_fail += 1;
                    let res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "enum");
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
                                    Matched(__pos, i) => Matched(__pos, { EnumType { identifier: Some(i), enumerators: Vec::new() } }),
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

fn __parse_enumerator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Enumerator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_identifier(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, i) => {
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
                                    let __seq_res = __parse_attribute_specifier_list(__input, __state, __pos, env);
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
                            Matched(__pos, a) => {
                                let __seq_res = __parse__(__input, __state, __pos, env);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = match __parse_enumerator_constant(__input, __state, __pos, env) {
                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                            Failed => Matched(__pos, None),
                                        };
                                        match __seq_res {
                                            Matched(__pos, e) => Matched(__pos, {
                                                env.add_symbol(&i.node.name, Symbol::Identifier);
                                                Enumerator { identifier: i, expression: e, extensions: a.unwrap_or_default() }
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
}

fn __parse_enumerator_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "=");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_constant_expression(__input, __state, __pos, env);
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

fn __parse_type_qualifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<TypeQualifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_type_qualifier0(__input, __state, __pos, env);
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

fn __parse_type_qualifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeQualifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let res = {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "const");
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
                                        let __seq_res = slice_eq(__input, __state, __pos, "__const");
                                        match __seq_res {
                                            Matched(__pos, e) => Matched(__pos, { e }),
                                            Failed => Failed,
                                        }
                                    }
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
                Matched(__pos, _) => Matched(__pos, { TypeQualifier::Const }),
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
                            let __seq_res = {
                                let __choice_res = slice_eq(__input, __state, __pos, "restrict");
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
                                                let __seq_res = {
                                                    let __seq_res = slice_eq(__input, __state, __pos, "__restrict");
                                                    match __seq_res {
                                                        Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                                            Matched(__newpos, _) => Matched(__newpos, ()),
                                                            Failed => Matched(__pos, ()),
                                                        },
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
                        Matched(__pos, _) => Matched(__pos, { TypeQualifier::Restrict }),
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
                                    let __seq_res = {
                                        let __choice_res = slice_eq(__input, __state, __pos, "volatile");
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
                                                        let __seq_res = {
                                                            let __seq_res = slice_eq(__input, __state, __pos, "__volatile");
                                                            match __seq_res {
                                                                Matched(__pos, _) => match slice_eq(__input, __state, __pos, "__") {
                                                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                                                    Failed => Matched(__pos, ()),
                                                                },
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
                                Matched(__pos, _) => Matched(__pos, { TypeQualifier::Volatile }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
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
                                                    __state.suppress_fail += 1;
                                                    let res = {
                                                        let __seq_res = slice_eq(__input, __state, __pos, "_Nonnull");
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
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => Matched(__pos, { TypeQualifier::Nonnull }),
                                        Failed => Failed,
                                    }
                                };
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
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
                                                            __state.suppress_fail += 1;
                                                            let res = {
                                                                let __seq_res = slice_eq(__input, __state, __pos, "_Null_unspecified");
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
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, { TypeQualifier::NullUnspecified }),
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
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
                                                                    __state.suppress_fail += 1;
                                                                    let res = {
                                                                        let __seq_res = slice_eq(__input, __state, __pos, "_Nullable");
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
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { TypeQualifier::Nullable }),
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __choice_res {
                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                    Failed => {
                                                        let __seq_res = {
                                                            __state.suppress_fail += 1;
                                                            let res = {
                                                                let __seq_res = slice_eq(__input, __state, __pos, "_Atomic");
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
                                                                            __state.suppress_fail += 1;
                                                                            let __assert_res = slice_eq(__input, __state, __pos, "(");
                                                                            __state.suppress_fail -= 1;
                                                                            match __assert_res {
                                                                                Failed => Matched(__pos, ()),
                                                                                Matched(..) => Failed,
                                                                            }
                                                                        };
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => Matched(__pos, { TypeQualifier::Atomic }),
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
            }
        }
    }
}

