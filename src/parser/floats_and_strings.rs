fn __parse_float_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Float> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = __parse_float_number(__input, __state, __pos, env);
        match __seq_res {
            Matched(__pos, n) => {
                let __seq_res = __parse_float_suffix(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, suffix) => Matched(__pos, {
                        let (base, number) = n;
                        Float { base: base, number: number.to_string().into_boxed_str(), suffix: suffix }
                    }),
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_float_number<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<(FloatBase, &'input str)> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let str_start = __pos;
                match __parse_float_decimal(__input, __state, __pos, env) {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, n) => Matched(__pos, { (FloatBase::Decimal, n) }),
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = __parse_ohx(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            let str_start = __pos;
                            match __parse_float_hexadecimal(__input, __state, __pos, env) {
                                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                                Failed => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, n) => Matched(__pos, { (FloatBase::Hexadecimal, n) }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_float_decimal<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
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
            };
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = slice_eq(__input, __state, __pos, ".");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = vec![];
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = __parse_dec(__input, __state, __pos, env);
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
                            };
                            match __seq_res {
                                Matched(__pos, _) => match __parse_float_decimal_exp(__input, __state, __pos, env) {
                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                    Failed => Matched(__pos, ()),
                                },
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
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = vec![];
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = __parse_dec(__input, __state, __pos, env);
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
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ".");
                            match __seq_res {
                                Matched(__pos, _) => match __parse_float_decimal_exp(__input, __state, __pos, env) {
                                    Matched(__newpos, _) => Matched(__newpos, ()),
                                    Failed => Matched(__pos, ()),
                                },
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
                            let mut __repeat_pos = __pos;
                            let mut __repeat_value = vec![];
                            loop {
                                let __pos = __repeat_pos;
                                let __step_res = __parse_dec(__input, __state, __pos, env);
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
                        };
                        match __seq_res {
                            Matched(__pos, _) => __parse_float_decimal_exp(__input, __state, __pos, env),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_float_decimal_exp<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = if __input.len() > __pos {
            let (__ch, __next) = char_range_at(__input, __pos);
            match __ch {
                'e' | 'E' => Matched(__next, ()),
                _ => __state.mark_failure(__pos, "[eE]"),
            }
        } else {
            __state.mark_failure(__pos, "[eE]")
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = match if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        '+' | '-' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[+-]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[+-]")
                } {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = vec![];
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = __parse_dec(__input, __state, __pos, env);
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
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_float_hexadecimal<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = {
                let mut __repeat_pos = __pos;
                loop {
                    let __pos = __repeat_pos;
                    let __step_res = __parse_hex(__input, __state, __pos, env);
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
            };
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = slice_eq(__input, __state, __pos, ".");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
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
                            };
                            match __seq_res {
                                Matched(__pos, _) => __parse_float_binary_exp(__input, __state, __pos, env),
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
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ".");
                            match __seq_res {
                                Matched(__pos, _) => __parse_float_binary_exp(__input, __state, __pos, env),
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
                        };
                        match __seq_res {
                            Matched(__pos, _) => __parse_float_binary_exp(__input, __state, __pos, env),
                            Failed => Failed,
                        }
                    }
                }
            }
        }
    }
}

fn __parse_float_binary_exp<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = if __input.len() > __pos {
            let (__ch, __next) = char_range_at(__input, __pos);
            match __ch {
                'p' | 'P' => Matched(__next, ()),
                _ => __state.mark_failure(__pos, "[pP]"),
            }
        } else {
            __state.mark_failure(__pos, "[pP]")
        };
        match __seq_res {
            Matched(__pos, _) => {
                let __seq_res = match if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        '+' | '-' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[+-]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[+-]")
                } {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = vec![];
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = __parse_dec(__input, __state, __pos, env);
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
                    }
                    Failed => Failed,
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_float_suffix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FloatSuffix> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            __state.suppress_fail += 1;
            let res = __parse_float_suffix_inner(__input, __state, __pos, env);
            __state.suppress_fail -= 1;
            res
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                __state.mark_failure(__pos, "float literal suffix");
                Failed
            }
        }
    }
}

fn __parse_float_suffix_inner<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FloatSuffix> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
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
            };
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse_float_format(__input, __state, __pos, env);
                    match __seq_res {
                        Matched(__pos, fmt) => Matched(__pos, { FloatSuffix { format: fmt, imaginary: true } }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                let __seq_res = __parse_float_format(__input, __state, __pos, env);
                match __seq_res {
                    Matched(__pos, fmt) => {
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
                        } {
                            Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                            Failed => Matched(__pos, None),
                        };
                        match __seq_res {
                            Matched(__pos, imag) => Matched(__pos, { FloatSuffix { format: fmt, imaginary: imag.is_some() } }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            }
        }
    }
}

fn __parse_float_format<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<FloatFormat> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = {
            let __seq_res = __parse_ts18661_float_suffix(__input, __state, __pos, env);
            match __seq_res {
                Matched(__pos, f) => Matched(__pos, { FloatFormat::TS18661Format(f) }),
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
                        Matched(__pos, _) => Matched(__pos, { FloatFormat::Float }),
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
                                    'l' | 'L' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[lL]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[lL]")
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { FloatFormat::LongDouble }),
                                Failed => Failed,
                            }
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => Matched(__pos, { FloatFormat::Double }),
                        }
                    }
                }
            }
        }
    }
}

fn __parse_character_constant<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<String> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let __seq_res = match if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        'L' | 'u' | 'U' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[LuU]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[LuU]")
                } {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "'");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = {
                                    let mut __repeat_pos = __pos;
                                    let mut __repeat_value = vec![];
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = __parse_character(__input, __state, __pos, env);
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
                                };
                                match __seq_res {
                                    Matched(__pos, _) => slice_eq(__input, __state, __pos, "'"),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, c) => Matched(__pos, { String::from(c) }),
            Failed => Failed,
        }
    }
}

fn __parse_character<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = if __input.len() > __pos {
            let (__ch, __next) = char_range_at(__input, __pos);
            match __ch {
                '\'' | '\\' | '\n' => __state.mark_failure(__pos, "[^'\\\n]"),
                _ => Matched(__next, ()),
            }
        } else {
            __state.mark_failure(__pos, "[^'\\\n]")
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_escape_sequence(__input, __state, __pos, env),
        }
    }
}

fn __parse_escape_sequence<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = slice_eq(__input, __state, __pos, "\\");
        match __seq_res {
            Matched(__pos, _) => {
                let __choice_res = if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        '\'' | '"' | '?' | '\\' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "['\"?\\abfnrtv]"),
                    }
                } else {
                    __state.mark_failure(__pos, "['\"?\\abfnrtv]")
                };
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = {
                            let mut __repeat_pos = __pos;
                            let mut __repeat_value = vec![];
                            loop {
                                let __pos = __repeat_pos;
                                if __repeat_value.len() >= 3 {
                                    break;
                                }
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
                        };
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __seq_res = slice_eq(__input, __state, __pos, "x");
                                match __seq_res {
                                    Matched(__pos, _) => {
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
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                    }
                }
            }
            Failed => Failed,
        }
    }
}

fn __parse_string_literal<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<Node<Vec<String>>> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let __seq_res = Matched(__pos, __pos);
            match __seq_res {
                Matched(__pos, l) => {
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
                                let __step_res = __parse_string_literal0(__input, __state, __pos, env);
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
            Matched(__pos, s) => Matched(__pos, { s }),
            Failed => Failed,
        }
    }
}

fn __parse_string_literal0<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<String> {
    #![allow(non_snake_case, unused)]
    {
        let __seq_res = {
            let str_start = __pos;
            match {
                let __seq_res = match __parse_encoding_prefix(__input, __state, __pos, env) {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = slice_eq(__input, __state, __pos, "\"");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = {
                                    let mut __repeat_pos = __pos;
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = __parse_string_char(__input, __state, __pos, env);
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
                                };
                                match __seq_res {
                                    Matched(__pos, _) => slice_eq(__input, __state, __pos, "\""),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            } {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, s) => Matched(__pos, { String::from(s) }),
            Failed => Failed,
        }
    }
}

fn __parse_encoding_prefix<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = slice_eq(__input, __state, __pos, "u8");
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => {
                if __input.len() > __pos {
                    let (__ch, __next) = char_range_at(__input, __pos);
                    match __ch {
                        'u' | 'U' | 'L' => Matched(__next, ()),
                        _ => __state.mark_failure(__pos, "[uUL]"),
                    }
                } else {
                    __state.mark_failure(__pos, "[uUL]")
                }
            }
        }
    }
}

fn __parse_string_char<'input>(__input: &'input str, __state: &mut ParseState<'input>, __pos: usize, env: &mut Env) -> RuleResult<()> {
    #![allow(non_snake_case, unused)]
    {
        let __choice_res = if __input.len() > __pos {
            let (__ch, __next) = char_range_at(__input, __pos);
            match __ch {
                '"' | '\\' | '\n' => __state.mark_failure(__pos, "[^\"\\\n]"),
                _ => Matched(__next, ()),
            }
        } else {
            __state.mark_failure(__pos, "[^\"\\\n]")
        };
        match __choice_res {
            Matched(__pos, __value) => Matched(__pos, __value),
            Failed => __parse_escape_sequence(__input, __state, __pos, env),
        }
    }
}

