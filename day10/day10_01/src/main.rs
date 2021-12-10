use std::io::Read;
use std::collections::VecDeque;
fn main() {
    let mut fd = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let tokens = parse(&contents);

    let score = part1(tokens);
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

fn get_error_score(x: &Token) -> u32 {
    match x {
        Token::CLOSING_PARAN() => 3,
        Token::CLOSING_SQUARE_BRACKET() => 57,
        Token::CLOSING_CURLY_BRACKET() => 1197,
        Token::CLOSING_POINTY_BRACK() => 25137,
        _ => panic!("got opening tag as invalid char")
    }
}

fn part1(tokens: Vec<Vec<Token>>) -> u32 {
    println!("{}", tokens.len());
    let mut score = 0;
    for line in tokens {
        //println!("{:?}", line);
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
                let mut maybe_missmatching = [
                    VALID_PARAN_PAIR,
                    VALID_SQUARE_PAIR,
                    VALID_CURLY_PAIR,
                    VALID_POINTY_PAIR,
                ]
                .into_iter()
                .map(|pair| {
                    if last_opening == pair.0 && token != pair.1 {
                        //println!("{:?} {:?} {:?}", pair, last_opening, token);
                        Some(token)
                    } else {
                        None
                    }
                })
                .filter(|it| it.is_some()).collect::<Vec<_>>();
                if maybe_missmatching.len() == 1 {
                    let x = maybe_missmatching[0].unwrap();
                    score += get_error_score(&x);
                    println!("{:?} {}", x, score);
                    break;
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
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
        let score = part1(tokens);
        println!("{}", score);
        assert_eq!(26397, score);
    }
}
