use std::collections::HashMap;
use std::collections::HashSet;

fn main()
{
    part1();
}

fn part1()
{
    let mut map: HashMap<(i32, i32), u32> = raw_input()
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .enumerate()
                .map(move |(x, digit)| ((x as i32, y as i32), digit))
        })
        .collect();

    let mut sum = 0;
    for i in 0..100 {
        println!("Step {}", i);
        for (_, energy) in map.iter_mut() {
            *energy += 1;
        }

        let mut flashes: HashSet<(i32, i32)> = HashSet::new();
        loop {
            let mut new_flashes: HashSet<(i32, i32)> = HashSet::new();

            for (position, energy) in map.iter() {
                if *energy > 9 && ! flashes.contains(position) {
                    new_flashes.insert(*position);
                }
            }
    
            if new_flashes.is_empty() {
                break;
            }

            for (x, y) in new_flashes {
                for inc_x in [-1, 0, 1] {
                    for inc_y in [-1, 0, 1] {
                        if inc_x == 0 && inc_y == 0 {
                            continue;
                        }

                        if let Some(energy) = map.get_mut(&(x + inc_x, y + inc_y)) {
                            *energy += 1;
                        }
                    }
                }

                flashes.insert((x, y));
            }
        }

        for position in flashes.iter() {
            let energy = map.get_mut(position).unwrap();
            *energy = 0;
        }

        println!("Step {}: {} flashes", i, flashes.len());
        sum += flashes.len();
        // print_map(&map);
    }

    
    println!("Part 1: {}", sum);
}

fn print_map(map: &HashMap<(i32, i32), u32>)
{
    for y in 0..5 {
        for x in 0..5 {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!();
    }
}

fn test_input_2() -> &'static str
{
    "
    11111
    19991
    19191
    19991
    11111
    "
}

fn test_input() -> &'static str
{
    "
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    "
}

fn raw_input() -> &'static str
{
    "
    6744638455
    3135745418
    4754123271
    4224257161
    8167186546
    2268577674
    7177768175
    2662255275
    4655343376
    7852526168
    "
}