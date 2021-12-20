fn main()
{
    println!("Part 1: {}", magnitude(add_lines(raw_input())));
}

fn add_lines(input: &'static str) -> Number
{
    let mut lines = input.trim().lines().map(|line| line.trim());
    let mut number = parse(lines.next().unwrap());

    for line in lines {
        number = add_and_reduce(number, parse(line));
    }

    number
}

#[derive(Clone)]
enum Number {
    Literal(u32),
    Pair(Box<Number>, Box<Number>),
}

fn parse(line: &'static str) -> Number
{
    let chars: Vec<char> = line.chars().collect();
    parse_chars(&chars).0
}

fn parse_chars(chars: &[char]) -> (Number, &[char])
{
    let split = chars.split_at(1);
    let head = split.0[0];
    let tail = split.1;

    if head == '[' {
        let (left, new_tail) = parse_chars(tail);
        let (comma, new_tail) = new_tail.split_at(1);
        assert_eq!(',', comma[0]);
        let (right, new_tail) = parse_chars(new_tail);
        let (braket, new_tail) = new_tail.split_at(1);
        assert_eq!(']', braket[0]);

        (Number::Pair(Box::new(left), Box::new(right)), new_tail)
    } else {
        (Number::Literal(head.to_digit(10).unwrap()), tail)
    }
}

fn print(number: &Number) -> String
{
    match number {
        Number::Literal(value) => format!("{}", value),
        Number::Pair(left, right) => format!("[{},{}]", print(&*left), print(&*right)),
    }
}

fn add(left: Number, right: Number) -> Number
{
    Number::Pair(Box::new(left), Box::new(right))
}

fn reduce(number: Number) -> Number
{
    let (number, explode) = explode(number, 4);

    match explode {
        Explode::Yes(_) => reduce(number),
        Explode::No => {
            let (number, splitted) = split(number);
            if splitted {
                reduce(number)
            } else {
                number
            }
        }
    }
}

enum Explode {
    No,
    Yes(ExplodeData),
}

struct ExplodeData {
    left: Option<u32>,
    right: Option<u32>,
}

fn explode(number: Number, depth: u32) -> (Number, Explode)
{
    match number {
        Number::Literal(_) => (number, Explode::No),
        Number::Pair(left, right) => {
            if depth > 0 {
                let (number_left, explode_left) = explode(*left.clone(), depth - 1);

                if let Explode::Yes(explode) = explode_left {
                    let new_right = if let Some(add_right) = explode.right {
                        Box::new(add_to_left(*right, add_right))
                    } else {
                        right
                    };

                    (Number::Pair(Box::new(number_left), new_right), Explode::Yes(ExplodeData { left: explode.left, right: None }))
                } else {
                    let (number_right, explode_right) = explode(*right, depth - 1);
                    if let Explode::Yes(explode) = explode_right {
                        let new_left = if let Some(add_left) = explode.left {
                            Box::new(add_to_right(*left, add_left))
                        } else {
                            left
                        };

                        (Number::Pair(new_left, Box::new(number_right)), Explode::Yes(ExplodeData { left: None, right: explode.right }))
                    } else {
                        (Number::Pair(Box::new(number_left), Box::new(number_right)), Explode::No)
                    }
                }
            } else {
                let left = unwrap_literal(*left);
                let right = unwrap_literal(*right);

                (Number::Literal(0), Explode::Yes(ExplodeData { left: Some(left), right: Some(right) }))
            }
        },
    }
}

fn split(number: Number) -> (Number, bool)
{
    match number {
        Number::Literal(value) if value >= 10 => {
            let left = (value as f32 / 2.0).floor() as u32;
            let right = (value as f32 / 2.0).ceil() as u32;
            (Number::Pair(Box::new(Number::Literal(left)), Box::new(Number::Literal(right))), true)
        },
        Number::Literal(value) => (Number::Literal(value), false),
        Number::Pair(left, right) => {
            let (new_left, splitted) = split(*left.clone());

            if splitted {
                (Number::Pair(Box::new(new_left), right), true)
            } else {
                let (new_right, splitted) = split(*right);
                (Number::Pair(left, Box::new(new_right)), splitted)
            }
        },
    }
}

fn add_to_left(number: Number, add: u32) -> Number
{
    match number {
        Number::Literal(value) => Number::Literal(value + add),
        Number::Pair(left, right) => Number::Pair(Box::new(add_to_left(*left, add)), right),
    }
}

fn add_to_right(number: Number, add: u32) -> Number
{
    match number {
        Number::Literal(value) => Number::Literal(value + add),
        Number::Pair(left, right) => Number::Pair(left, Box::new(add_to_right(*right, add))),
    }
}

fn unwrap_literal(number: Number) -> u32
{
    match number {
        Number::Literal(value) => value,
        Number::Pair(_, _) => panic!(),
    }
}

fn add_and_reduce(left: Number, right: Number) -> Number
{
    reduce(add(left, right))
}

fn magnitude(number: Number) -> u32
{
    match number {
        Number::Literal(value) => value,
        Number::Pair(left, right) => magnitude(*left) * 3 + magnitude(*right) * 2,
    }
}


#[test]
fn it_works()
{
    assert_eq!("[1,2]", print(&parse("[1,2]")));
    assert_eq!("[[1,2],3]", print(&parse("[[1,2],3]")));
    assert_eq!("[9,[8,7]]", print(&parse("[9,[8,7]]")));
    assert_eq!("[[1,9],[8,5]]", print(&parse("[[1,9],[8,5]]")));
    assert_eq!("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", print(&parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")));
    assert_eq!("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", print(&parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")));
    assert_eq!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", print(&parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")));

    assert_eq!("[[1,2],[[3,4],5]]", print(&add(parse("[1,2]"), parse("[[3,4],5]"))));
    assert_eq!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", print(&add(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]"))));
    
    assert_eq!("[[1,2],[[3,4],5]]", print(&add_and_reduce(parse("[1,2]"), parse("[[3,4],5]"))));
    assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", print(&add_and_reduce(parse("[[[[4,3],4],4],[7,[[8,4],9]]]"), parse("[1,1]"))));

    assert_eq!("[[[[0,9],2],3],4]", print(&reduce(parse("[[[[[9,8],1],2],3],4]"))));
    assert_eq!("[7,[6,[5,[7,0]]]]", print(&reduce(parse("[7,[6,[5,[4,[3,2]]]]]"))));
    assert_eq!("[[6,[5,[7,0]]],3]", print(&reduce(parse("[[6,[5,[4,[3,2]]]],1]"))));
    assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", print(&reduce(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))));

    assert_eq!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", print(&explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), 4).0));

    assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", print(&add_lines("
    [1,1]
    [2,2]
    [3,3]
    [4,4]
    ")));

    assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", print(&add_lines("
    [1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    ")));

    assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", print(&add_lines("
    [1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    [6,6]
    ")));

    assert_eq!("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", print(&add_lines("
    [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]
    ")));


    assert_eq!(143, magnitude(parse("[[1,2],[[3,4],5]]")));
    assert_eq!(1384, magnitude(parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
    assert_eq!(445, magnitude(parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")));
    assert_eq!(791, magnitude(parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")));
    assert_eq!(1137, magnitude(parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")));
    assert_eq!(3488, magnitude(parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));

}

fn raw_input() -> &'static str
{
    "
    [[6,[[9,4],[5,1]]],[[[6,5],[9,4]],2]]
    [[7,3],[[3,[5,5]],8]]
    [8,[[5,0],[[0,2],3]]]
    [[[8,7],[[2,0],[7,5]]],1]
    [[[2,[6,1]],[7,[6,1]]],[[7,3],1]]
    [[2,[9,[0,0]]],[[[9,7],1],0]]
    [[[[8,4],[2,3]],[[6,4],4]],0]
    [[[1,3],1],[[3,8],[[2,3],[9,5]]]]
    [[7,[5,9]],[[7,[9,1]],[3,[9,6]]]]
    [[[5,3],5],[[[8,8],[5,6]],[6,5]]]
    [3,[[4,1],3]]
    [[[5,[2,0]],[[9,5],[9,2]]],[[[1,7],[6,9]],[[6,3],[8,6]]]]
    [[[[9,3],[2,4]],[6,9]],[[[9,7],1],[[1,9],[2,9]]]]
    [3,[[6,1],8]]
    [[[[8,8],8],[[3,9],[9,3]]],[[8,8],[[7,1],[6,5]]]]
    [[[8,9],[[2,7],6]],[[[2,9],[8,4]],[1,6]]]
    [[4,[[4,4],0]],[[8,[1,8]],[9,[7,3]]]]
    [[[[3,0],[7,2]],[[9,5],[9,5]]],[5,[0,[5,7]]]]
    [5,[1,[[4,0],[8,5]]]]
    [[0,0],[[[9,8],1],[[5,2],[4,6]]]]
    [[5,8],[6,[[5,2],1]]]
    [[1,[[1,4],8]],8]
    [[[[1,7],[7,1]],[4,[8,0]]],0]
    [[[[5,9],0],[0,8]],[2,[[6,2],2]]]
    [2,[4,3]]
    [[[[4,0],[2,2]],7],[[8,7],[[8,1],1]]]
    [[[[6,0],[1,6]],[2,[6,2]]],[[9,6],[7,[8,2]]]]
    [[3,5],[[9,[4,0]],[[6,5],[1,0]]]]
    [[[[6,0],7],[8,[0,1]]],[[7,6],[[7,1],[9,6]]]]
    [[3,[[6,4],4]],[0,[[3,5],[8,6]]]]
    [[8,[[1,8],0]],[1,[[0,1],[6,2]]]]
    [[6,[5,[5,4]]],9]
    [[[[0,7],3],[[7,7],[1,2]]],[8,[2,1]]]
    [[[7,[1,4]],[5,[9,8]]],[1,8]]
    [[[0,7],[[3,6],[2,4]]],[[7,4],1]]
    [[[5,[8,2]],[[4,9],[5,3]]],4]
    [[5,[[3,3],0]],8]
    [7,[2,1]]
    [[3,8],[[[5,3],8],[[3,4],6]]]
    [[[2,[0,9]],[0,5]],0]
    [[6,[7,6]],[[[2,6],2],[[8,9],5]]]
    [[[0,0],[[1,9],[0,6]]],[[5,[8,8]],[[6,9],[3,7]]]]
    [[[[4,6],[8,4]],[2,[3,8]]],[8,0]]
    [[0,0],[2,[[6,2],6]]]
    [[[6,0],3],8]
    [[[[6,1],[4,8]],[2,[3,0]]],7]
    [[[0,[1,8]],[[8,1],6]],3]
    [2,[0,2]]
    [[[[9,6],8],[[1,9],[7,8]]],[[[0,6],[8,8]],[6,[2,3]]]]
    [[0,[6,[7,4]]],[[[0,9],[2,3]],[[8,8],0]]]
    [[[0,1],[7,[4,9]]],[[3,9],8]]
    [[[1,9],7],[[0,5],[5,[7,9]]]]
    [[[9,[2,5]],2],[7,[1,[7,7]]]]
    [[[[0,4],[7,3]],2],5]
    [[8,[7,4]],[[[8,2],[7,3]],[1,[7,8]]]]
    [[[0,4],[[3,7],9]],6]
    [[5,[[9,2],[7,0]]],[[8,2],[[1,4],9]]]
    [2,[[[9,6],9],[2,3]]]
    [[5,[[3,5],[3,8]]],[4,[2,9]]]
    [[[5,2],[4,[4,1]]],[[[1,0],[8,7]],[[8,7],8]]]
    [[4,[4,[0,9]]],[[1,8],4]]
    [[[3,[4,0]],[[8,8],[1,6]]],[[4,0],[1,2]]]
    [[[1,[1,8]],2],[[6,2],[9,[8,5]]]]
    [9,[[[8,8],[8,3]],[3,[1,3]]]]
    [[[2,[4,5]],[4,1]],[1,[[8,6],[1,5]]]]
    [[0,[5,[7,6]]],[[8,6],[[9,9],1]]]
    [[[5,[5,2]],2],[[[1,4],[3,7]],[4,3]]]
    [[5,[[9,8],0]],[7,[[0,8],[7,8]]]]
    [[[[8,0],6],[2,1]],[[[6,3],[3,1]],[[7,6],[7,2]]]]
    [[[3,3],6],[2,[[8,4],5]]]
    [[[6,[5,3]],[[6,4],3]],[[[4,8],0],[[0,6],[1,4]]]]
    [[[3,[6,4]],2],[[[8,8],4],[[8,6],6]]]
    [[[[6,9],1],[3,8]],[[5,[4,6]],2]]
    [[5,6],3]
    [[[5,[8,6]],[[4,2],[1,1]]],[[[0,7],[6,3]],[9,[7,7]]]]
    [[7,[[4,0],6]],[[4,[6,4]],8]]
    [[5,[[2,0],[9,4]]],[[[4,6],1],[[2,8],[8,5]]]]
    [[[[3,5],[0,4]],[[5,0],3]],[[1,[8,9]],7]]
    [[[[6,6],6],[[6,6],[4,3]]],0]
    [[5,[2,5]],[6,[[7,8],2]]]
    [[[7,[5,5]],[[7,4],[6,7]]],0]
    [[[3,3],3],[[1,9],[0,[9,2]]]]
    [[9,[4,1]],[6,[2,[9,6]]]]
    [[4,7],[9,[3,0]]]
    [[[8,2],[[9,8],[4,2]]],[[2,[3,7]],[7,[3,1]]]]
    [[[[1,8],2],[0,[6,5]]],[[[2,7],[8,6]],[[8,9],[8,5]]]]
    [[[7,[2,9]],[9,0]],5]
    [[5,[2,[1,5]]],[0,7]]
    [4,[[0,[0,3]],[[0,5],[9,0]]]]
    [0,[[4,4],[[8,4],[3,8]]]]
    [[[[4,9],0],[[4,4],9]],[[[6,1],[8,9]],[7,[2,3]]]]
    [[[[4,2],[7,4]],0],[[5,[0,6]],[[0,5],4]]]
    [[[1,0],8],[[[2,8],[2,9]],3]]
    [[6,[1,[9,9]]],[2,2]]
    [[[8,[6,7]],[6,[6,6]]],[[[2,3],5],0]]
    [[[7,[6,9]],[[7,8],[2,8]]],[[4,[5,1]],5]]
    [[[[6,3],[1,4]],7],[[9,1],[3,1]]]
    [5,[[8,5],[[7,5],4]]]
    [[4,[[4,0],0]],[6,[1,1]]]
    [[[5,[9,2]],[9,0]],[[5,[5,7]],4]]
    "
}