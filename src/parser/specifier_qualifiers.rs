fn __parse_specifier_qualifiers<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Vec<Node<SpecifierQualifier>>> {
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
                        let __step_res = {
                            let __seq_res = Matched(__pos, __pos);
                            match __seq_res {
                                Matched(__pos, l) => {
                                    let __seq_res = __parse_specifier_qualifier_qualifier0(__input, __state, __pos, env);
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
                Matched(__pos, before) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let __seq_res = Matched(__pos, __pos);
                                match __seq_res {
                                    Matched(__pos, l) => {
                                        let __seq_res = __parse_specifier_qualifier_unique_type0(__input, __state, __pos, env);
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
                                Matched(__pos, single) => {
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
                                                                    let __seq_res = __parse_specifier_qualifier_qualifier0(__input, __state, __pos, env);
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
                                                Matched(__pos, after) => Matched(__pos, {
                                                    let mut before = before;
                                                    before.push(single);
                                                    before.extend(after);
                                                    before
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
                                        let __seq_res = __parse_specifier_qualifier_qualifier0(__input, __state, __pos, env);
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
                    Matched(__pos, before) => {
                        let __seq_res = __parse__(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = {
                                    let __seq_res = Matched(__pos, __pos);
                                    match __seq_res {
                                        Matched(__pos, l) => {
                                            let __seq_res = __parse_specifier_qualifier_nonunique_type0(__input, __state, __pos, env);
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
                                    Matched(__pos, single) => {
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
                                                                let __choice_res = {
                                                                    let __seq_res = Matched(__pos, __pos);
                                                                    match __seq_res {
                                                                        Matched(__pos, l) => {
                                                                            let __seq_res = __parse_specifier_qualifier_nonunique_type0(__input, __state, __pos, env);
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
                                                                match __choice_res {
                                                                    Matched(__pos, __value) => Matched(__pos, __value),
                                                                    Failed => {
                                                                        let __seq_res = Matched(__pos, __pos);
                                                                        match __seq_res {
                                                                            Matched(__pos, l) => {
                                                                                let __seq_res = __parse_specifier_qualifier_qualifier0(__input, __state, __pos, env);
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
                                                    Matched(__pos, after) => Matched(__pos, {
                                                        let mut before = before;
                                                        before.push(single);
                                                        before.extend(after);
                                                        before
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
    }
}

fn __parse_specifier_qualifier_unique_type0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SpecifierQualifier> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_type_specifier_unique(__input, __state, __pos, env);
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
            Matched(__pos, s) => Matched(__pos, { SpecifierQualifier::TypeSpecifier(s) }),
            Failed => Failed,
        }
    }
}

fn __parse_specifier_qualifier_nonunique_type0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SpecifierQualifier> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse_type_specifier_nonunique(__input, __state, __pos, env);
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
            Matched(__pos, s) => Matched(__pos, { SpecifierQualifier::TypeSpecifier(s) }),
            Failed => Failed,
        }
    }
}

fn __parse_specifier_qualifier_qualifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<SpecifierQualifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_type_qualifier(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, q) => Matched(__pos, { SpecifierQualifier::TypeQualifier(q) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = {
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
                            let __seq_res = __parse_attribute_specifier(__input, __state, __pos, env);
                            match __seq_res {
                                Matched(__pos, e) => Matched(__pos, { e }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                };
                match __seq_res {
                    Matched(__pos, e) => Matched(__pos, { SpecifierQualifier::Extension(e) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_struct_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<StructDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = match __parse_declarator(__input, __state, __pos, env) {
                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                Failed => Matched(__pos, None),
            };
            match __seq_res {
                Matched(__pos, d) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ":");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = __parse__(__input, __state, __pos, env);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse_constant_expression(__input, __state, __pos, env);
                                            match __seq_res {
                                                Matched(__pos, e) => {
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
                                                        Matched(__pos, a) => Matched(__pos, { StructDeclarator { declarator: d.map(|d| with_ext(d, a)), bit_width: Some(e) } }),
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
                let __seq_res = __parse_declarator(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, d) => {
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
                                    Matched(__pos, a) => Matched(__pos, { StructDeclarator { declarator: Some(with_ext(d, a)), bit_width: None } }),
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

