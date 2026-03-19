pub fn constant<'input>(__input: &'input str, env: &mut Env) -> ParseResult<Constant> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_constant(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}

pub fn string_literal<'input>(__input: &'input str, env: &mut Env) -> ParseResult<Node<Vec<String>>> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_string_literal(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}

pub fn expression<'input>(__input: &'input str, env: &mut Env) -> ParseResult<Box<Node<Expression>>> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_expression(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}

pub fn declaration<'input>(__input: &'input str, env: &mut Env) -> ParseResult<Node<Declaration>> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_declaration(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}

pub fn statement<'input>(__input: &'input str, env: &mut Env) -> ParseResult<Box<Node<Statement>>> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_statement(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}

pub fn translation_unit<'input>(__input: &'input str, env: &mut Env) -> ParseResult<TranslationUnit> {
    #![allow(non_snake_case, unused)]
    let mut __state = ParseState::new();
    match __parse_translation_unit(__input, &mut __state, 0, env) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError { line: __line, column: __col, offset: __state.max_err_pos, expected: __state.expected })
}
