use std::collections::VecDeque;
use std::io::Read;
fn main() {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let tokens = parse(&contents);

    let score = part2(tokens);
    println!("{}", score);
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
enum Token {
    OPEN_PARAN(),
    CLOSING_PARAN(),
    OPEN_SQUARE_BRACKET(),
    CLOSING_SQUARE_BRACKET(),
    OPEN_CURLY_BRACKET(),
    CLOSING_CURLY_BRACKET(),
    OPEN_POINTY_BRACK(),
    CLOSING_POINTY_BRACK(),
}

const VALID_PARAN_PAIR: (Token, Token) = (Token::OPEN_PARAN(), Token::CLOSING_PARAN());
const VALID_SQUARE_PAIR: (Token, Token) = (
    Token::OPEN_SQUARE_BRACKET(),
    Token::CLOSING_SQUARE_BRACKET(),
);
const VALID_CURLY_PAIR: (Token, Token) =
    (Token::OPEN_CURLY_BRACKET(), Token::CLOSING_CURLY_BRACKET());
const VALID_POINTY_PAIR: (Token, Token) =
    (Token::OPEN_POINTY_BRACK(), Token::CLOSING_POINTY_BRACK());

fn parse(input: &str) -> Vec<Vec<Token>> {
    let mut ret = Vec::new();
    let lines = input.split('\n');
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let chars = line.chars();
        let mut token = Vec::new();
        for cc in chars {
            let mapped = match cc {
                '(' => Token::OPEN_PARAN(),
                ')' => Token::CLOSING_PARAN(),
                '[' => Token::OPEN_SQUARE_BRACKET(),
                ']' => Token::CLOSING_SQUARE_BRACKET(),
                '{' => Token::OPEN_CURLY_BRACKET(),
                '}' => Token::CLOSING_CURLY_BRACKET(),
                '<' => Token::OPEN_POINTY_BRACK(),
                '>' => Token::CLOSING_POINTY_BRACK(),
                _ => panic!("invalid char {}", cc),
            };
            token.push(mapped);
        }
        ret.push(token);
    }
    ret
}

fn get_completion_score(x: &Token) -> u128 {
    match x {
        Token::OPEN_PARAN() => 1,
        Token::OPEN_SQUARE_BRACKET() => 2,
        Token::OPEN_CURLY_BRACKET() => 3,
        Token::OPEN_POINTY_BRACK() => 4,
        _ => panic!("got closing tag as invalid char"),
    }
}

fn part2(tokens: Vec<Vec<Token>>) -> u128 {
    let mut scores = Vec::new();
    'line_loop: for line in tokens {
        let mut stack = VecDeque::new();
        for token in line {
            if matches!(
                token,
                Token::OPEN_PARAN()
                    | Token::OPEN_SQUARE_BRACKET()
                    | Token::OPEN_CURLY_BRACKET()
                    | Token::OPEN_POINTY_BRACK()
            ) {
                stack.push_back(token);
            } else {
                let last_opening = match stack.pop_back() {
                    None => panic!("closng token right at the beginning"),
                    Some(lt) => lt,
                };
                let maybe_missmatching = [
                    VALID_PARAN_PAIR,
                    VALID_SQUARE_PAIR,
                    VALID_CURLY_PAIR,
                    VALID_POINTY_PAIR,
                ]
                .into_iter()
                .map(|pair| {
                    if last_opening == pair.0 && token != pair.1 {
                        Some(token)
                    } else {
                        None
                    }
                })
                .filter(|it| it.is_some());
                if maybe_missmatching.count() == 1 {
                    continue 'line_loop;
                }
            }
        }
        if !stack.is_empty() {
            let mut score = 0;
            while let Some(tt) = stack.pop_back() {
                score = (score * 5) + get_completion_score(&tt);
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let tokens = parse(&input);
        let score = part2(tokens);
        println!("{}", score);
        assert_eq!(288957, score);
    }
}
