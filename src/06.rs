fn main()
{
    part1();
    part2();
}

fn part1()
{
    let total = compute(80);
    println!("Part 1: {}", total);
}

fn part2()
{
    let total = compute(256);
    println!("Part 2: {}", total);
}

fn compute(days: u64) -> u64
{
    let mut lanternfishes = [0; 9];
    for digit in raw_input().split(',').map(|digit| digit.parse::<usize>().unwrap()) {
        lanternfishes[digit] += 1;
    }

    for _day in 0..days {
        let mut new_lanternfishes = [0; 9];
        for (index, count) in lanternfishes.iter().enumerate() {
            if index == 0 {
                new_lanternfishes[6] += count;
                new_lanternfishes[8] += count;
            } else {
                new_lanternfishes[index - 1] += count;
            }
        }

        lanternfishes = new_lanternfishes;
    }

    lanternfishes.iter().sum()
}

fn test_input() -> &'static str
{
    "3,4,3,1,2"
}

fn raw_input() -> &'static str
{
    "1,4,1,1,1,1,1,1,1,4,3,1,1,3,5,1,5,3,2,1,1,2,3,1,1,5,3,1,5,1,1,2,1,2,1,1,3,1,5,1,1,1,3,1,1,1,1,1,1,4,5,3,1,1,1,1,1,1,2,1,1,1,1,4,4,4,1,1,1,1,5,1,2,4,1,1,4,1,2,1,1,1,2,1,5,1,1,1,3,4,1,1,1,3,2,1,1,1,4,1,1,1,5,1,1,4,1,1,2,1,4,1,1,1,3,1,1,1,1,1,3,1,3,1,1,2,1,4,1,1,1,1,3,1,1,1,1,1,1,2,1,3,1,1,1,1,4,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,5,1,1,1,2,2,1,1,3,5,1,1,1,1,3,1,3,3,1,1,1,1,3,5,2,1,1,1,1,5,1,1,1,1,1,1,1,2,1,2,1,1,1,2,1,1,1,1,1,2,1,1,1,1,1,5,1,4,3,3,1,3,4,1,1,1,1,1,1,1,1,1,1,4,3,5,1,1,1,1,1,1,1,1,1,1,1,1,1,5,2,1,4,1,1,1,1,1,1,1,1,1,1,1,1,1,5,1,1,1,1,1,1,1,1,2,1,4,4,1,1,1,1,1,1,1,5,1,1,2,5,1,1,4,1,3,1,1"
}