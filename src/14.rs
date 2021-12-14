use std::collections::HashMap;

fn main()
{
    part1();
}

type Pair = (char, char);
type Rules = HashMap<Pair, HashMap<u32, Vec<Pair>>>;

fn part1()
{
    let result = compute_result(raw_input(), 10);
    println!("Part 1: {}", result);
}

fn compute_string(input: &'static str, steps: u32) -> String
{
    let (template, mut rules) = build_input(input);

    let mut new_pairs = vec![];
    for pair in template {
        let result = get_result(rules, pair, steps);

        rules = result.0;
        for other_pair in result.1 {
            new_pairs.push(other_pair);
        }
    }

    pairs_to_string(new_pairs)
}

fn compute_result(input: &'static str, steps: u32) -> u32
{
    let mut chars_counts: HashMap<char, u32> = HashMap::new();

    let result = compute_string(input, steps);

    for char in result.chars() {
        let entry = chars_counts.entry(char).or_insert(0);
        *entry += 1;
    }

    let min = chars_counts.values().min().unwrap();
    let max = chars_counts.values().max().unwrap();

    max - min
}

fn build_input(input: &'static str) -> (Vec<Pair>, Rules)
{
    let (template_as_string, rules_as_string) = input.trim().split_once("\n\n").unwrap();

    let mut rules: Rules = HashMap::new();
    for line in rules_as_string.lines().map(|line| line.trim()) {
        let (from_as_string, to_as_string) = line.split_once(" -> ").unwrap();

        let mut from_chars = from_as_string.chars();
        let from_a = from_chars.next().unwrap();
        let from_b = from_chars.next().unwrap();
        let from: Pair = (from_a, from_b);
        let to: char = to_as_string.chars().next().unwrap();

        let entry = rules.entry(from).or_insert_with(HashMap::new);
        entry.insert(1, vec![(from_a, to), (to, from_b)]);
    }


    let chars: Vec<char> = template_as_string.chars().collect();

    let template: Vec<Pair> = chars.windows(2).map(|digits| (digits[0], digits[1])).collect();

    (template, rules)
}

fn get_result(mut rules: Rules, pair: Pair, steps: u32) -> (Rules, Vec<Pair>)
{
    let instructions = rules.get(&pair).unwrap();

    let mut i = steps;
    let result: Vec<Pair> = loop {
        if let Some(result) = instructions.get(&i) {
            break result.clone();
        }

        i -= 1;
    };

    if i == steps {
        return (rules, result);
    }

    let mut new_pairs = vec![];
    for other_pair in result {
        let other_result = get_result(rules, other_pair, steps - i);

        rules = other_result.0;
        for other_other_pair in other_result.1 {
            new_pairs.push(other_other_pair);
        }
    }

    (rules, new_pairs)
}

#[test]
fn test_part1()
{
    let result = get_result(build_input(test_input()).1, ('N', 'N'), 1);
    assert_eq!(vec![('N', 'C'), ('C', 'N')], result.1);

    let result = get_result(build_input(test_input()).1, ('N', 'N'), 2);
    assert_eq!(vec![('N', 'B'), ('B', 'C'), ('C', 'C'), ('C', 'N')], result.1);

    let result = get_result(build_input(test_input()).1, ('N', 'N'), 2);
    assert_eq!("NBCCN", pairs_to_string(result.1));

    let result = get_result(build_input(test_input()).1, ('N', 'N'), 3);
    assert_eq!("NBBBCNCCN", pairs_to_string(result.1));

    let result = get_result(build_input(test_input()).1, ('N', 'N'), 4);
    assert_eq!("NBBNBNBBCCNBCNCCN", pairs_to_string(result.1));

    let result = compute_string(test_input(), 4);
    assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", result);

    let result = compute_result(test_input(), 10);
    assert_eq!(1588, result);
}

fn pairs_to_string(pairs: Vec<Pair>) -> String
{
    let mut s = String::with_capacity(pairs.len() + 1);

    let mut first = true;
    for (a, b) in pairs {
        if first {
            s.push(a);
            first = false;
        }

        s.push(b);
    }

    s
}

fn test_input() -> &'static str
{
    "
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
    "
}

fn raw_input() -> &'static str
{
    "
    FSKBVOSKPCPPHVOPVFPC

    BV -> O
    OS -> P
    KP -> P
    VK -> S
    FS -> C
    OK -> P
    KC -> S
    HV -> F
    HC -> K
    PF -> N
    NK -> F
    SC -> V
    CO -> K
    PO -> F
    FB -> P
    CN -> K
    KF -> N
    NH -> S
    SF -> P
    HP -> P
    NP -> F
    OV -> O
    OP -> P
    HH -> C
    FP -> P
    CS -> O
    SK -> O
    NS -> F
    SN -> S
    SP -> H
    BH -> B
    NO -> O
    CB -> N
    FO -> N
    NC -> C
    VF -> N
    CK -> C
    PC -> H
    BP -> B
    NF -> O
    BB -> C
    VN -> K
    OH -> K
    CH -> F
    VB -> N
    HO -> P
    FH -> K
    PK -> H
    CC -> B
    VH -> B
    BF -> N
    KS -> V
    PV -> B
    CP -> N
    PB -> S
    VP -> V
    BO -> B
    HS -> H
    BS -> F
    ON -> B
    HB -> K
    KH -> B
    PP -> H
    BN -> C
    BC -> F
    KV -> K
    VO -> P
    SO -> V
    OF -> O
    BK -> S
    PH -> V
    SV -> F
    CV -> H
    OB -> N
    SS -> H
    VV -> B
    OO -> V
    CF -> H
    KB -> F
    NV -> B
    FV -> V
    HK -> P
    VS -> P
    FF -> P
    HN -> N
    FN -> F
    OC -> K
    SH -> V
    KO -> C
    HF -> B
    PN -> N
    SB -> F
    VC -> B
    FK -> S
    KK -> N
    FC -> F
    NN -> P
    NB -> V
    PS -> S
    KN -> S
    "
}