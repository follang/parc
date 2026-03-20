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

pub fn translation_unit_resilient<'input>(__input: &'input str, env: &mut Env) -> TranslationUnit {
    #![allow(non_snake_case, unused)]
    // Try strict parse first
    let mut __state = ParseState::new();
    match __parse_translation_unit(__input, &mut __state, 0, env) {
        Matched(__pos, __value) if __pos == __input.len() => {
            return __value;
        }
        _ => {}
    }

    // Strict parse failed — use recovery loop
    let mut items: Vec<Node<ExternalDeclaration>> = Vec::new();
    let mut pos = 0usize;

    // Skip leading whitespace/directives
    {
        let mut __state = ParseState::new();
        match __parse__(__input, &mut __state, pos, env) {
            Matched(new_pos, _) => { pos = new_pos; }
            _ => {}
        }
    }

    while pos < __input.len() {
        let mut __state = ParseState::new();
        let start = pos;
        match __parse_external_declaration(__input, &mut __state, pos, env) {
            Matched(new_pos, decl) => {
                items.push(Node::new(decl, Span::span(start, new_pos)));
                pos = new_pos;
                // Skip whitespace between declarations
                let mut __state2 = ParseState::new();
                match __parse__(__input, &mut __state2, pos, env) {
                    Matched(new_pos, _) => { pos = new_pos; }
                    _ => {}
                }
            }
            Failed => {
                // Skip to next sync point
                match __skip_to_sync_point(__input, pos) {
                    Some(new_pos) => {
                        pos = new_pos;
                        // Skip whitespace after sync point
                        let mut __state2 = ParseState::new();
                        match __parse__(__input, &mut __state2, pos, env) {
                            Matched(new_pos, _) => { pos = new_pos; }
                            _ => {}
                        }
                    }
                    None => break,
                }
            }
        }
    }

    TranslationUnit(items)
}

fn __skip_to_sync_point(input: &str, pos: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = pos;
    let mut brace_depth: i32 = 0;

    while i < len {
        match bytes[i] {
            b'{' => {
                brace_depth += 1;
                i += 1;
            }
            b'}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                    i += 1;
                    if brace_depth == 0 {
                        // Skip optional semicolons after closing brace
                        while i < len && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\r') {
                            i += 1;
                        }
                        if i < len && bytes[i] == b';' {
                            i += 1;
                        }
                        return Some(i);
                    }
                } else {
                    i += 1;
                }
            }
            b';' if brace_depth == 0 => {
                i += 1;
                return Some(i);
            }
            b'/' if i + 1 < len && bytes[i + 1] == b'/' => {
                while i < len && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if i + 1 < len && bytes[i + 1] == b'*' => {
                i += 2;
                while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    i += 1;
                }
                if i + 1 < len {
                    i += 2;
                }
            }
            b'\'' | b'"' => {
                let quote = bytes[i];
                i += 1;
                while i < len && bytes[i] != quote {
                    if bytes[i] == b'\\' && i + 1 < len {
                        i += 1;
                    }
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    None
}
