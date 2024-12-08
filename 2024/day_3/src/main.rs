

const MUL_REGEX: &str = r"mul\((\d+)\,(\d+)\)";
const DO_REGEX: &str = r"do\(\)";
const DONT_REGEX: &str = r"don't\(\)";

fn get_regexes() -> Vec<regex::Regex> {
    let mul_re = regex::Regex::new(MUL_REGEX).unwrap();
    let do_re = regex::Regex::new(DO_REGEX).unwrap();
    let dont_re = regex::Regex::new(DONT_REGEX).unwrap();
    vec![mul_re, do_re, dont_re]
}

fn parse_mul(input: &str) -> Option<(i32, i32, usize)> {
    let re = regex::Regex::new(MUL_REGEX).unwrap();
    let cap = re.captures(input)?;
    let a = cap.get(1)?.as_str().parse::<i32>().unwrap();
    let b = cap.get(2)?.as_str().parse::<i32>().unwrap();
    Some((a, b, cap.get(0)?.as_str().len()))
}

fn find_first_match(input: &str) -> Option<regex::Match> {
    let regexes = get_regexes();
    let finds: Vec<Option<regex::Match<'_>>> = regexes.iter().map(|re| re.find(input))
        .filter(|x| x.is_some()).collect();
    if finds.is_empty() {
        return None;
    }
    for ele in finds.iter() {
        println!("{:?}", ele);
    }
    let first_find = finds.iter().min_by_key(|x| x.unwrap().start());
    println!("first find: {:?} ", first_find);
    *first_find.unwrap()
}

fn with_regex() {
    let input = std::fs::read_to_string("src/input_big.txt").unwrap();
    let mut sum = 0;
    let mut i = 0;
    let mut do_adding = true;
    while i < input.len() {
        let l = find_first_match(&input[i..]);
        if l.is_none() {
            break;
        }
        let m = l.unwrap();
        println!("m: {:?}", m.as_str());
        match m.as_str() {
            "do()" => {
                do_adding = true;
                println!("do() found");
            }
            "don't()" => {
                do_adding = false;
                println!("don't() found");
            }
            _ => {
                let (a, b, _len) = parse_mul(&m.as_str()).unwrap();
                if do_adding {
                    println!("{} * {} = ", a, b);
                    sum += a * b;
                }
            }
        }
        i += m.end();
    }
    println!("sum: {}", sum);
}


fn main() {
    with_regex();
}
