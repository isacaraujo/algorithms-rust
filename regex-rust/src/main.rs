#[derive(Debug, Clone, PartialEq)]
enum Token {
    Literal(char),
    Dot,
    Star(Box<Token>),
    Plus(Box<Token>),
    Question(Box<Token>),
    StartAnchor,
    EndAnchor,
}

fn parse_pattern(pattern: &str) -> Vec<Token> {
    let chars = pattern.chars().collect::<Vec<char>>();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '.' => tokens.push(Token::Dot),
            '^' => tokens.push(Token::StartAnchor),
            '$' => tokens.push(Token::EndAnchor),
            '*' if !tokens.is_empty() => {
                let prev = tokens.pop().unwrap();
                tokens.push(Token::Star(Box::new(prev)));
            },
            '+' if !tokens.is_empty() => {
                let prev = tokens.pop().unwrap();
                tokens.push(Token::Plus(Box::new(prev)));
            },
            '?' if !tokens.is_empty() => {
                let prev = tokens.pop().unwrap();
                tokens.push(Token::Question(Box::new(prev)));
            },
            c => tokens.push(Token::Literal(c)),
        }
        i += 1;
    }

    tokens
}

fn match_here(tokens: &[Token], text: &[char], pos: usize) -> bool {
    if tokens.is_empty() {
        return true;
    }

    if let Token::EndAnchor = tokens[0] {
        return tokens.len() == 1 && pos == text.len();
    }

    match &tokens[0] {
        Token::Star(inner) => {
            if match_here(&tokens[1..], text, pos) {
                return true;
            }

            if pos < text.len() && match_token(inner.as_ref(), text[pos]) {
                return match_here(tokens, text, pos + 1);
            }
            false
        }
        Token::Plus(inner) => {
            if pos < text.len() && match_token(inner, text[pos]) {
                let star_token = Token::Star(inner.clone());
                return match_here(&[star_token], text, pos + 1) &&
                    match_here(&tokens[1..], text, pos + 1);
            }
            false
        }
        Token::Question(inner) => {
            if pos < text.len() && match_token(inner, text[pos]) {
                if match_here(&tokens[1..], text, pos + 1) {
                    return true;
                }
            }

            match_here(&tokens[1..], text, pos)
        }
        token => {
            if pos >= text.len() {
                return false;
            }
            if match_token(token, text[pos]) {
                return match_here(&tokens[1..], text, pos + 1);
            }
            false
        }
    }
}

fn match_token(token: &Token, c: char) -> bool {
    match token {
        Token::Literal(ch) => *ch == c,
        Token::Dot => true,
        _ => false,
    }
}

fn regex_match(pattern: &str, text: &str) -> bool {
    let tokens = parse_pattern(pattern);
    let text_chars = text.chars().collect::<Vec<char>>();

    if let Some(Token::StartAnchor) = tokens.first() {
        return match_here(&tokens[1..], &text_chars, 0);
    }

    for start_pos in 0..=text_chars.len() {
        if match_here(&tokens, &text_chars, start_pos) {
            return true
        }
    }

    false
}

fn main() {
    let tests = vec![
        // (pattern, text, expected)
        ("abc", "abc", true),
        ("abc", "abcd", true),
        ("abc", "zabc", true),
        ("abc", "zabc", true),
        ("^abc", "abcd", true),
        ("abc$", "zabc", true),
        ("abc$", "abcd", false),
        ("a.c", "abc", true),
        ("a.c", "adc", true),
        ("a.c", "ac", false),
        ("a*", "", true),
        ("a*", "aaa", true),
        ("a*b", "aaaab", true),
        ("a*b", "b", true),
        ("a+b", "ab", true),
        ("a+b", "aaaab", true),
        ("a+b", "b", false),
        ("a?b", "ab", true),
        ("a?b", "b", true),
        ("a?b", "aab", true),
        (".*", "anything", true),
        ("a?b", "ab", true),
        ("^h.*o$", "hello", true),
        ("^h.*o$", "hi there yo", true),
    ];

    for (pattern, text, expected) in tests {
        let result = regex_match(pattern, text);
        let status = if result == expected {
            "\x1B[32m\x1B[1mPASSED\x1B[0m"
        } else {
            "\x1B[31m\x1B[1mFAILED\x1B[0m"
        };

        println!("{} Pattern: {} Text: {} => {}", status, pattern, text, result);
    }
}
