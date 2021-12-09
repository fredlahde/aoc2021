use std::fs::File;
use std::io::Read;

fn main() {
    let mut fd  = File::open("input").unwrap();
    let mut contents = String::new();
    fd.read_to_string(&mut contents).unwrap();
    let parsed = parse(&contents);
    let ans = part_one(&parsed);
    println!("part one {}", ans);
}

const COUNT_ONE: usize = 2;
const COUNT_FOUR: usize = 4;
const COUNT_SEVEN: usize = 3;
const CONT_EIGHT: usize = 7;

fn part_one(outputs: &[Vec<&str>]) -> usize {
    outputs
        .iter()
        .flat_map(|it| it.iter())
        .map(|it| it.len())
        .filter(|it| matches!(*it, COUNT_ONE | COUNT_FOUR | COUNT_SEVEN | CONT_EIGHT))
        .count()
}

fn parse<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
    let mut ret = Vec::new();
    let lines = input.split('\n');
    for line in lines {
        if line.is_empty() {
            continue
        }
        let mut split = line.split('|');
        let _signals = split.next().unwrap();
        let outputs = split.next().unwrap();
        let outputs = outputs.split_whitespace().collect::<Vec<_>>();
        ret.push(outputs);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let parsed = parse(&input);
        let part_one_ans = part_one(&parsed);
        assert_eq!(26, part_one_ans);
    }
}
