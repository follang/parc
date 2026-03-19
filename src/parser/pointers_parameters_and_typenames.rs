fn __parse_pointer<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<DerivedDeclarator>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_pointer0(__input, __state, __pos, env);
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

fn __parse_pointer0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<DerivedDeclarator> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let __choice_res = slice_eq(__input, __state, __pos, "*");
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
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
                                let __seq_res = slice_eq(__input, __state, __pos, "^");
                                match __seq_res {
                                    Matched(__pos, e) => Matched(__pos, { e }),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, t) => {
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
                                                let __seq_res = __parse_pointer_qualifier(__input, __state, __pos, env);
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
                            Matched(__pos, q) => Matched(__pos, {
                                if t == "^" {
                                    DerivedDeclarator::Block(q)
                                } else {
                                    DerivedDeclarator::Pointer(q)
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
}

fn __parse_pointer_qualifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<PointerQualifier> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_type_qualifier(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, q) => Matched(__pos, { PointerQualifier::TypeQualifier(q) }),
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
                    Matched(__pos, e) => Matched(__pos, { PointerQualifier::Extension(e) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_ellipsis<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Ellipsis> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = slice_eq(__input, __state, __pos, ",");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse__(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, "...");
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { Ellipsis::Some }),
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
            Failed => Matched(__pos, { Ellipsis::None }),
        }
    }
}

fn __parse_parameter_declaration<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<ParameterDeclaration>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_parameter_declaration0(__input, __state, __pos, env);
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

fn __parse_parameter_declaration0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<ParameterDeclaration> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_declaration_specifiers(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, s) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_parameter_declarator(__input, __state, __pos, env);
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
                                            Matched(__pos, a) => Matched(__pos, { ParameterDeclaration { specifiers: s, declarator: d, extensions: a.unwrap_or_default() } }),
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

fn __parse_parameter_declarator<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Option<Node<Declarator>>> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_declarator(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, d) => Matched(__pos, {
                    env.handle_declarator(&d, Symbol::Identifier);
                    Some(d)
                }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = __parse_abstract_declarator(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, d) => Matched(__pos, { Some(d) }),
                        Failed => Failed,
                    }
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => Matched(__pos, { None }),
                }
            }
        }
    }
}

fn __parse_type_name<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<TypeName>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_type_name0(__input, __state, __pos, env);
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

fn __parse_type_name0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<TypeName> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_specifier_qualifiers(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, s) => {
                let __seq_res = __parse__(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = match __parse_abstract_declarator(__input, __state, __pos, env) {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, d) => Matched(__pos, { TypeName { specifiers: s, declarator: d } }),
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

