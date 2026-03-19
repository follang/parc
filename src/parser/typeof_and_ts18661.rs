fn __parse_typeof_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeSpecifier> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            __state.suppress_fail += 1;
            let res = {
                let __seq_res = {
                    let __choice_res = slice_eq(__input, __state, __pos, "typeof");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __seq_res = slice_eq(__input, __state, __pos, "__typeof");
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
                                                    let __seq_res = __parse_typeof_specifier0(__input, __state, __pos, env);
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
                                                            Matched(__pos, _) => Matched(__pos, { TypeSpecifier::TypeOf(e) }),
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

fn __parse_typeof_specifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeOf> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
                Matched(__pos, e) => Matched(__pos, { TypeOf::Expression(e) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = __parse_type_name(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, t) => Matched(__pos, { TypeOf::Type(t) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_ts18661_float_type_specifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TS18661FloatType> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = __parse_ts18661_binary_float(__input, __state, __pos, env);
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_ts18661_decimal_float(__input, __state, __pos, env),
        }
    }
}

fn __parse_ts18661_binary_float<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TS18661FloatType> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "_Float");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse_ts18661_binary_width(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, width) => {
                        let __seq_res = match slice_eq(__input, __state, __pos, "x") {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, extended) => Matched(__pos, { ts18661_float(true, width, extended.is_some()) }),
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

fn __parse_ts18661_binary_width<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<usize> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let __choice_res = slice_eq(__input, __state, __pos, "16");
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = slice_eq(__input, __state, __pos, "32");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = slice_eq(__input, __state, __pos, "64");
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => slice_eq(__input, __state, __pos, "128"),
                                }
                            }
                        }
                    }
                }
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => Matched(__pos, { n.parse().unwrap() }),
            Failed => Failed,
        }
    }
}

fn __parse_ts18661_decimal_float<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TS18661FloatType> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "_Decimal");
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = __parse_ts18661_decimal_width(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, width) => {
                        let __seq_res = match slice_eq(__input, __state, __pos, "x") {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, extended) => Matched(__pos, { ts18661_float(false, width, extended.is_some()) }),
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

fn __parse_ts18661_decimal_width<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<usize> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let __choice_res = slice_eq(__input, __state, __pos, "32");
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = slice_eq(__input, __state, __pos, "64");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => slice_eq(__input, __state, __pos, "128"),
                        }
                    }
                }
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => Matched(__pos, { n.parse().unwrap() }),
            Failed => Failed,
        }
    }
}

fn __parse_ts18661_float_suffix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TS18661FloatType> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let __choice_res = slice_eq(__input, __state, __pos, "df");
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => slice_eq(__input, __state, __pos, "DF"),
                }
            };
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { ts18661_float(false, 32, false) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "dd");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => slice_eq(__input, __state, __pos, "DD"),
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { ts18661_float(false, 64, false) }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let __seq_res = {
                                let __choice_res = slice_eq(__input, __state, __pos, "dl");
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => slice_eq(__input, __state, __pos, "DL"),
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { ts18661_float(false, 128, false) }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = {
                                    let __seq_res = if __input.len() > __pos {
                                        let (__ch, __next) = char_range_at(__input, __pos);
                                        match __ch {
                                            'f' | 'F' => Matched(__next, ()),
                                            _ => __state.mark_failure(__pos, "[fF]"),
                                        }
                                    } else {
                                        __state.mark_failure(__pos, "[fF]")
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse_ts18661_binary_width(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, width) => {
                                                    let __seq_res = match slice_eq(__input, __state, __pos, "x") {
                                                        Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                        Failed => Matched(__pos, None),
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, extended) => Matched(__pos, { ts18661_float(true, width, extended.is_some()) }),
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
                                        let __seq_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                'd' | 'D' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[dD]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[dD]")
                                        };
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse_ts18661_decimal_width(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, width) => {
                                                        let __seq_res = match slice_eq(__input, __state, __pos, "x") {
                                                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                                            Failed => Matched(__pos, None),
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, extended) => Matched(__pos, { ts18661_float(false, width, extended.is_some()) }),
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

fn __parse_clang_guard<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    match {
        if env.extensions_clang {
            Ok(())
        } else {
            Err("clang extensions disabled")
        }
    } {
        Ok(res) => Matched(__pos, res),
        Err(expected) => {
            __state.mark_failure(__pos, expected);
            Failed
        }
    }
}

