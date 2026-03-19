fn __parse_identifier<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Identifier>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = Matched(__pos, __pos);
        match __seq_res {
            Matched(__pos, l) => {
                let __seq_res = __parse_identifier0(__input, __state, __pos, env);
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

fn __parse_identifier0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Identifier> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
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
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => {
                match {
                    if !env.reserved.contains(n) {
                        Ok(Identifier { name: n.into() })
                    } else {
                        Err("identifier")
                    }
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
}

fn __parse_ohx<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "0");
        match __seq_res {
            Matched(__pos, _) => {
                if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        'x' | 'X' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[xX]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[xX]")
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_obb<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "0");
        match __seq_res {
            Matched(__pos, _) => {
                if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        'b' | 'B' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[bB]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[bB]")
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_dec<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    if __input.len() > __pos {
        let (__ch, __next) = char_range_at(__input, __pos);
        match __ch {
            '0'..='9' => Matched(__next, ()),
            _ => __state.mark_failure(__pos, "[0-9]"),
        }
    } else {
        __state.mark_failure(__pos, "[0-9]")
    }
}

fn __parse_oct<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    if __input.len() > __pos {
        let (__ch, __next) = char_range_at(__input, __pos);
        match __ch {
            '0'..='7' => Matched(__next, ()),
            _ => __state.mark_failure(__pos, "[0-7]"),
        }
    } else {
        __state.mark_failure(__pos, "[0-7]")
    }
}

fn __parse_hex<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    if __input.len() > __pos {
        let (__ch, __next) = char_range_at(__input, __pos);
        match __ch {
            '0'..='9' | 'a'..='f' | 'A'..='F' => Matched(__next, ()),
            _ => __state.mark_failure(__pos, "[0-9a-fA-F]"),
        }
    } else {
        __state.mark_failure(__pos, "[0-9a-fA-F]")
    }
}

fn __parse_bin<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    if __input.len() > __pos {
        let (__ch, __next) = char_range_at(__input, __pos);
        match __ch {
            '0'..='1' => Matched(__next, ()),
            _ => __state.mark_failure(__pos, "[0-1]"),
        }
    } else {
        __state.mark_failure(__pos, "[0-1]")
    }
}

fn __parse_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Constant> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                __state.suppress_fail += 1;
                let __assert_res = if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        '0'..='9' | '.' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[0-9.]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[0-9.]")
                };
                __state.suppress_fail -= 1;
                match __assert_res {
                    Matched(_, __value) => Matched(__pos, __value),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse_numeric_constant(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, c) => Matched(__pos, { c }),
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
                    let __assert_res = if __input.len() > __pos {
                        let (__ch, __next) = char_range_at(__input, __pos);
                        match __ch {
                            '\'' | 'u' | 'U' | 'L' => Matched(__next, ()),
                            _ => __state.mark_failure(__pos, "['uUL]"),
                        }
                    } else {
                        __state.mark_failure(__pos, "['uUL]")
                    };
                    __state.suppress_fail -= 1;
                    match __assert_res {
                        Matched(_, __value) => Matched(__pos, __value),
                        Failed => Failed,
                    }
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = __parse_character_constant(__input, __state, __pos, env);
                        match __seq_res {
                            Matched(__pos, c) => Matched(__pos, { Constant::Character(c) }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_numeric_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Constant> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_float_constant(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, c) => Matched(__pos, { Constant::Float(c) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = __parse_integer_constant(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, c) => Matched(__pos, { Constant::Integer(c) }),
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_integer_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Integer> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_integer_number(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, n) => {
                let __seq_res = __parse_integer_suffix(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, suffix) => Matched(__pos, {
                        let (base, number) = n;
                        Integer { base: base, number: number.to_owned().into_boxed_str(), suffix: suffix }
                    }),
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_integer_number<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<(IntegerBase, &'input str)> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __seq_res = if __input.len() > __pos {
                        let (__ch, __next) = char_range_at(__input, __pos);
                        match __ch {
                            '1'..='9' => Matched(__next, ()),
                            _ => __state.mark_failure(__pos, "[1-9]"),
                        }
                    } else {
                        __state.mark_failure(__pos, "[1-9]")
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let mut __repeat_pos = __pos;
                            loop {
                                let __pos = __repeat_pos;
                                let __step_res = __parse_dec(__input, __state, __pos, env);
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
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, n) => Matched(__pos, { (IntegerBase::Decimal, n) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __choice_res = {
                    let __seq_res = __parse_ohx(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let str_start = __pos;
                                match {
                                    let mut __repeat_pos = __pos;
                                    let mut __repeat_value = vec![];
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = __parse_hex(__input, __state, __pos, env);
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
                                        Matched(__repeat_pos, ())
                                    } else {
                                        Failed
                                    }
                                } {
                                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, n) => Matched(__pos, { (IntegerBase::Hexadecimal, n) }),
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
                            let __seq_res = slice_eq(__input, __state, __pos, "0");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        let str_start = __pos;
                                        match {
                                            let mut __repeat_pos = __pos;
                                            let mut __repeat_value = vec![];
                                            loop {
                                                let __pos = __repeat_pos;
                                                let __step_res = __parse_oct(__input, __state, __pos, env);
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
                                                Matched(__repeat_pos, ())
                                            } else {
                                                Failed
                                            }
                                        } {
                                            Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                                            Failed => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, n) => Matched(__pos, { (IntegerBase::Octal, n) }),
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
                                                let __seq_res = __parse_obb(__input, __state, __pos, env);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = {
                                                            let str_start = __pos;
                                                            match {
                                                                let mut __repeat_pos = __pos;
                                                                let mut __repeat_value = vec![];
                                                                loop {
                                                                    let __pos = __repeat_pos;
                                                                    let __step_res = __parse_bin(__input, __state, __pos, env);
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
                                                                    Matched(__repeat_pos, ())
                                                                } else {
                                                                    Failed
                                                                }
                                                            } {
                                                                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, n) => Matched(__pos, { (IntegerBase::Binary, n) }),
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
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __seq_res = {
                                            let str_start = __pos;
                                            match slice_eq(__input, __state, __pos, "0") {
                                                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                                                Failed => Failed,
                                            }
                                        };
                                        match __seq_res {
                                            Matched(__pos, n) => Matched(__pos, { (IntegerBase::Decimal, n) }),
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

fn __parse_integer_suffix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<IntegerSuffix> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            __state.suppress_fail += 1;
            let res = __parse_integer_suffix_inner(__input, __state, __pos, env);
            __state.suppress_fail -= 1;
            res
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                __state.mark_failure(__pos, "integer suffix");
                Failed
            }
        }
    }
}

fn __parse_integer_suffix_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<IntegerSuffix> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let mut __repeat_pos = __pos;
                loop {
                    let __pos = __repeat_pos;
                    let __step_res = {
                        let __choice_res = if __input.len() > __pos {
                            let (__ch, __next) = char_range_at(__input, __pos);
                            match __ch {
                                'u' | 'U' | 'l' | 'L' => Matched(__next, ()),
                                _ => __state.mark_failure(__pos, "[uUlL]"),
                            }
                        } else {
                            __state.mark_failure(__pos, "[uUlL]")
                        };
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
                                        let __seq_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                'i' | 'I' | 'j' | 'J' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[iIjJ]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[iIjJ]")
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
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, s) => match { int_suffix(s) } {
                Ok(res) => Matched(__pos, res),
                Err(expected) => {
                    __state.mark_failure(__pos, expected);
                    Failed
                }
            },
            Failed => Failed,
        }
    }
}

