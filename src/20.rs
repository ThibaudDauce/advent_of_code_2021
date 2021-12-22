use std::collections::HashSet;

fn main()
{
    println!("Part 1: {}", number_of_lit_pixels(raw_input(), 2));
    println!("Part 2: {}", number_of_lit_pixels(raw_input(), 50));
}

fn number_of_lit_pixels(input: &'static str, steps: u32) -> usize
{
    let (algorithm, map_as_string) = input.split_once("\n\n").unwrap();

    let algorithm: Vec<bool> = algorithm.trim().chars().map(|char| {
        match char {
            '.' => false,
            '#' => true,
            _ => panic!(),
        }
    }).collect();

    let mut map: HashSet<(i32, i32)> = map_as_string
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .map(|char| {
                    match char {
                        '.' => false,
                        '#' => true,
                        _ => panic!(),
                    }
                })
                .enumerate()
                .filter_map(move |(x, lit)| {
                    match lit {
                        true => Some((x as i32, y as i32)),
                        false => None,
                    }
                })
        })
        .collect();

    let mut outside_lit = false;
    print_map(&map, outside_lit);

    for step in 0..steps {
        println!("{}", step);
        let min_x = map.iter().map(|(x, _)| x).min().unwrap() - 4;
        let max_x = map.iter().map(|(x, _)| x).max().unwrap() + 4;
    
        let min_y = map.iter().map(|(_, y)| y).min().unwrap() - 4;
        let max_y = map.iter().map(|(_, y)| y).max().unwrap() + 4;

        let new_outside_lit = match outside_lit {
            true => algorithm[511],
            false => algorithm[0],
        };

        let mut new_map = HashSet::new();

        for x in min_x..max_x {
            for y in min_y..max_y {
                let mut decimal: usize = 0;
                let mut pow = 0;
                for inc_y in [1, 0, -1] {
                    for inc_x in [1, 0, -1] {
                        let inside_map = map.contains(&(x + inc_x, y + inc_y));

                        if x == 2 && y == 2 {
                            dbg!(x + inc_x, y + inc_y, inside_map);
                        }

                        let lit = match outside_lit {
                            true => ! inside_map,
                            false => inside_map,
                        };

                        if lit {
                            decimal += 2_usize.pow(pow);
                        }

                        pow += 1;
                    }
                }

                if x == 2 && y == 2 {
                    println!("\t{},{}: {}", x, y, decimal);
                }

                if new_outside_lit && ! algorithm[decimal] {
                    new_map.insert((x, y));
                }

                if ! new_outside_lit && algorithm[decimal] {
                    new_map.insert((x, y));
                }
            }
        }

        map = new_map;
        outside_lit = new_outside_lit;
    
        print_map(&map, outside_lit);

    }

    if outside_lit {
        panic!();
    }

    map.len()
}

fn print_map(map: &HashSet<(i32, i32)>, outside_lit: bool)
{
    let min_x = map.iter().map(|(x, _)| x).min().unwrap() - 4;
    let max_x = map.iter().map(|(x, _)| x).max().unwrap() + 4;

    let min_y = map.iter().map(|(_, y)| y).min().unwrap() - 4;
    let max_y = map.iter().map(|(_, y)| y).max().unwrap() + 4;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let inside_map = map.contains(&(x, y));
            let lit = match outside_lit {
                true => ! inside_map,
                false => inside_map,
            };

            if lit {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn it_works()
{
    assert_eq!(35, number_of_lit_pixels(test_input(), 2));
    assert_eq!(3351, number_of_lit_pixels(test_input(), 50));
}

fn test_input() -> &'static str
{
    "
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
    "
}

fn raw_input() -> &'static str
{
    "
    ###.#..##.#...#.##.#..#...#####..###....###..##..####.#..#.##.#.#...##.##.##.#...#....##..#..####...####..######..#.##..###......#..##.##.##...###.#..#.#.##.##.#..#####....####...#..##.###.#....#..#..#..###.#.#.##.#.#.#.######...#.###..####.....##..#..#.#....#.####.##.##..#.##.....##...#..#.#..####.##.#..#..#.###.##....##.#..........###.#..#.....#.##.##......##.....##...#........#.#......###...#####....##.#.#..###...#.#.###.##.#..#..###.#......#......####..#.#.##..#..#...####.#.#.#.#...#.####....##.###.....

    .##.#.###..#.....###..#.#.#.#.#.......#...#...##...####.######.#.##..#.##.#.#.###.#.##.#..####.#....
    .#..#...##.##..######....##.#.##.###...#.##.#####..##..#.###.####..###.#.##..###..#....#.#..#...#.##
    ##.##.##..#...#....#######.#..##..#..##...#######..###..#.##....####.....#....#..#..##.##.#..##..###
    #.##..##.....##...###..###.#...#..#..####..#.##....####.#...#.##..#.##.#.##.#.#.#######.##.#...##..#
    .##......#.#.###.####..#..#.####..#.####.#####.###.##.#.#..##..####.#...##..####.####.....####.#..##
    .#.#####.###....#.###.#.##.#.#.#.##.######....#.##.###..#.##.#..####.##..##.#.#..#.####.#####.#....#
    ..##.####...#.#.##.....#.#.##......#...###.#...##.##..#..####..##.##.#...#....##.#####..#.#...#.#...
    ###.###.##..##..#...#######...##.##...###..###.#.#.#.#..##.#..###....#.#..#..#....#.....#..#.....##.
    #...##...#.####.##....#....##..#.##.##.##..#..##..#.....#####....#.#######..##....####..#.#.#.#...##
    .#...#..#.##.#..#.#.....####.#.#..#..#####....##..##.#.####.##..####...#.###...#...##.##.#.#####.##.
    .#####.##.##..#.####.#.#...######.####.######..#..#..#..#.##...#####.####....##...#..#.#....##.###.#
    ...#......##......#.#..##.#.###.####.#.##.#...###.#.##...#.##....#.#.#....###.....#...#..##..#######
    ##......###.####...#...####..........#.########...##.#.##....#.#######.##..####....#...#.#.#....#..#
    .#.#.....#.#.#####.##.#.##.#..#.#..####.#.##.##.##..####...####...##..######.##..#..#.......###.##.#
    .##.....######...#..#.#..#.##....#.#..##.###.###.##.#.####.###.##...##....#.#..######.##..#.#..#.#.#
    .#.####.#...#.#.#.#.##..##...#....#...#..#....#..######..#..##.#.###.#..##.#.#.#......#.##..##.#...#
    ..#.##.#.#...###.###.#...####.###.#..####.#.#.#.#.##..#...#.#.##..##..#...####.####..##....###...##.
    ##.#.####.#....##.###...##.##.##.####.#.##...####.#....#####..##.#......#......#....#..#.#.##..#....
    ##.#.#.#..#.####..#..###.###.##..#######...#...#...#..#....#..#######.#..####..#.######..#.#..#.....
    #.#.##.##.#.######.##.##.####..#.###...##...#.##.#.###..#...###...##.#....####.....#..##.###..##.##.
    #.......###..#.#....###...##.###.##....#.##..##.....#.##.#.##..###...#..###.###.##.##....##.#.##.##.
    .#.##...#..##.#.....##.##...#.#..#..#.#.....###.#.#..###.##.#.#.##.###.####..##......##.#..###.#.#.#
    ##.#######.###...#..###..##..#....##.#....#.#..###.#####...#..##...#...#..#.....#..####..##..#.#...#
    ..###.#.##.#.#..######...#.##...#.#.#..#..#.....##..##...#.#.####.#.#..##..###....##.##...###.#.###.
    .##...###..##..#......##.#.##..#.##..#...#####...#.##.#.###..###......#.##.#.######..##........###.#
    ...#.#.####..######.####.###.#.####..#...##..#.#.##.##.##.#.##.##..#.....#.....##...#.##.##..##.####
    ....###.#.#...##.##..##.#...#...#..####.###.#.#...#.##.####.#.....###.##.##..###...#.#.#..#..#.#.#..
    ##.....#########.##.###..#...#.###..#.#...######.#..#.#.#..##.....#.#....###..#.##...#...#.##.#.###.
    #..##.###..#...#.#.#..##.....###.##.########..###.#.....#####..######...#..#.#.##..##.####..#.#.#.#.
    .####.####...###..##.#########...##.....#..####...#.##....###...#.##...#..#.#...#.###..###..##.###..
    ##....#.....##..#.###...#.......#.######.#.....#.##..#..#####.#.#.#.#.#........####...#.#.#.#.#.##.#
    ####.#...####...###......#....#..##..###.#.#..##.#....#.#######..#...#..#.#....####.##.####.##.##.#.
    #.#..##..#.#..###....#..#######.#..###..#.##...##.#...#.#....#....##..#...####.##.#####.####..#.#..#
    #..#.###.#.########.#.######........####...#.####.#######.##..######..##.#.#...##..#.##.#####..###.#
    #.##.##.#..#..#.#.####.......#...##..###.#..#####..#..####.##.#...#...##.##..###...##.###.#...#.....
    #..###...##.#.####..#.##...#.#.#.......##..#..###.#....##...#..#.##.......######.##..######...###.##
    ..##.##...#......##...#...#.....#..#....#.#...#....###.##....#..###...#.##....#####..#..###....#.#..
    ....#####..#..####..##.####.######.#..#.#..#..###..##....##.....###.#..##.#.###......#...#.####.#...
    ..#.###.#..##..#####..###.##.#..#.#..#.###.######.#..###.##...#..#####.###.##.#..#####..###..##.###.
    #..#.....#.#...#.#..###.#####...###....##...##.###.#..##.#.#..######..#..##..##..#...#.#...#.#..#.##
    #..#.#..#.##..##.###..###..#..#.#.#####.##....###.#.###..#..##....#..##.#.#.######....#.##.#.#...#..
    #.#...#..##.###.#.##....###....#######..##.##...##.....##...##.#..#.##...####..####....###...##.#.##
    #..###.##.#.#.#.##..#######.#.#...#..#.##...######...###.####.####.#..#.#.#.##.#.###.##.#.#..#.####.
    ..###.#...##.##..#.##.#..#.##.#.#.##.#..##.#.#.#.#...##....####.....####.#.##.......##..#....#.##...
    ###...#####..#..##.#...####....#.#.#######.#.#####........##..#.##.#..#.#.#.###.##.###..#.#.##..#.#.
    .#..#.##......#..#..##..###..#.##....#.#..##.###......##.#.........#..###...#.#.##..#.#.#..#.......#
    ##....#..##.##.#..#...##....######..####.#..#.###..#.#.###...#..#..#.#..##.....#..###.###.#..#..###.
    ..#####.#.###.....#.#.#...###....#####.##.####...##...######..###....##.#.....##.###...#.#.#...##.#.
    #...##..###......#..#.####.#.#.#.###.#.##.###...#.#.#.......#######.##...#..#.####...##...##....#...
    #..###.###.......###.#.#.#.#.##.###..#..#..###..#.#.###..#.#.##...#..#.####...###.##...#.#....#..###
    ##...##.#...#..######.#....#.######.#.#.#####.#.##.#..#..##.###.##.#.#.###.##.....####..#.#.###.....
    ..#.#.....#####..#.#..#.#.#####..#.#.#.......###.#..#########..##..###.#....#..#..##.##.##.#..#.#.##
    ###.#.##..#..#####..####.###...#..#...#.##..##.#..#.#....#####.###..#######.#####.##.#......###....#
    ..##...#####.#.#..##.......##..##...#..##.###..##...###.#.########.....###.....###.........#.###.##.
    #...#.#.#.#..##..#.#..#####...#####...##..##..#....##.#....#..##.##....###.#...#..#....#.####..#..##
    .####....#..#......#........#.#####..#.##...#.####.#....##.###..#...#.#....###.#.##...#..##.#...####
    .##..#.#.#....#.##.#.###.#.###...#..#....#.###.#...#..#..#..##..###....#.##.####..###.##....#.#....#
    .###.#.##..#..#.#..##....#..####...##..####.##.#.#.##.#.#.##....##.#..#.#..##.##.###........######.#
    ..#.#....##...##.#.....#..#.#..#.####...###.##.#.#.#.#.#.##.#.#.#..#.#..#.##..#...#.##.#.#.#..#.###.
    ...#.#.####...#.#.####.##.#...###.###.#.#.##..##.###.#..####.#..#..##..#.##.######....####.#.#..####
    ....###...#.#.#..#..#.#.##..#...###.##..##....##..#.....##...######.#...#######...#.###......#..#.##
    #.##.#.##.######..###.###..#...#.#...#..##.#####..#.#..##.###.#..#..######...###.#.#...##...###.###.
    #.##.#.#..#.##.#.#.##..##.#....####.#..###.....#.#..#######....###.#.###.###...####..####...##.###.#
    ##.#..#.........#.##..#.#..#.....##.#.##..##..##.#..###.#####.#.###.###.##.....#.####..#.#..#.#..###
    ##..#.###.##..##.#..#..#..###...#.##...##...##....##..#..#.#####..#..###.#..###.#.##.#...#...##.####
    ..#..##.#####.#...#.....#..#.##.#.##......###..#..#..#..#.####.##........##.##..#...###.#.##.##..##.
    .#..#.###.#....###...###..#........#.#...##.#.#####..##.#.##.#..#.#.##.##..#...#.#.##.#.##.....####.
    ....##.#...#.#..####..#...##...#..#.####..##..##.#..#.####...###.###...##.##..##..#...####...#...#..
    ....#..........###....#......#.#####.#..#..#..#####.##.###.##..###.###..#..####....#.##..##...#..###
    ##.#.####...#.#.#...###.#########.#....##..#...####...#...#...###..###.##.#...##..##..##....#.####.#
    ##.##..#.#..#...##...##.###..#.##..##..###.##..#.#.#.#.#.###...#..#.#.#.###....#..###..#.###.###.#.#
    #.##.###.#.#.#.#.####.#.#.##..###...###.###.#...##.##.#.##########.#.####.#.##...###.#.#.##.....####
    #######.#.###..##.###..###.#.###.#####.###..#.#.#.#.###..#...#..#.###..#.##.#..######..#...##.#####.
    .#.#.###.##.#.#..#..#.##.....###...##.#..##.##......##.#...#.###.##..###....#..#..#..####.##.#.#.#.#
    #.#.#.##..#.#..#.##..##..#...##.##..#....##.##...###..#######.#.#.###..####...#.#.###.####.###..####
    .##....####..#.####.###..#..####.#.###..##.###.#..#.##.#.###.####.#..#.#.....#.##.##.#..##.#.#.#..##
    #####..##..#.#.#....##..###.#.##...##..##..#..#.######.#.#.#.#..#.#.#####.###.#....#...##.##...####.
    .#.#####.....#..#..#.#.##..#.#....##.#.##.#..##.####.#.###.#.....#..##......#..##.#..##.....#.#.##.#
    #.###..#...#....##.##.##..##..#...#.#...#..#.##.......#.######.###...#.####.#.#....#.#.#.###.#.#..##
    #.##..###...#.#...##.##..#......###.###..##.#...#.#...##..#..#.#..##.##...#...###.#.##...##..#...#..
    .####....##.##..#.#.....#.#..#..#.##....#...#..###.#.##.#.#.##..#.#.####.##.#.###..###.##...##...##.
    .#..##....#..#.#.#.#.#.#.....#.#.......#.#.#...#.#..#.#.#...##.......#.##....##.#.#...#....#.#.#....
    .####.###..#.#.#.#.#..##...###...#...###..######.....#...#.#.#####..#.#.#.#.#...###.##.#.##....#.##.
    #......#####.#...#..###....#..#..#..#..#####..#...#...#........##.#.#.##.##....##.#########.#..##..#
    .##.#.#..####...#..#.#.......##......##...#.##....##.#..###..##......#.#..#.#..####..##.#...#...###.
    #.###.....##.######.##.####.#..#..######.##...##.###..###..##..#.#.####..#.##.##.##.#..#.####.######
    ##..###.###.....#....###...#........####...#..##.#.#.#...#...##...#..####....###.##.#.##.#.#.#.#..#.
    ..#...#####...##.#####...#..####..####.#.#####..####.#.#######.#....###....##.#..#.#.#..#....#.#.###
    .#..###..##.....####.#.#........#.###...#######..####.##.#..#.#####.#.......##.##.#.##..##......##.#
    #.#...##..###.#...######..#.##.#.#.#######......###....##.######.##.##.#.###.#..###.##.#...#####..##
    ...##.##.####.###.#####....#..#......#....##.##..##.#.#...#.#...#.####.#####...##...#.###.....###.##
    #......##.....###.#..##....###.##.#..#....#.#.#.#..#..##..####..#...#...#....####.#....########....#
    ...###..#.....##..#####.#..#.#####..####..####.##.#.#..####.#..####.#..##..##...###..#.##..###..####
    ##.##...#.########.#.#...#.##...#.#.#.#.###.#..#..#.####.#...#....#.#.###..##....#...#...##.###...#.
    ......#..###.#.#......#.#....##.###..#####..##.#.#..#####.##....#.#.#.#..##..#..##.#.#.........#.##.
    ..#.###..#.###.#.#.###.##..#.##....#....##.#...#.#.#...####.###....##.####........##..#..#..####..#.
    .#.....#..#..#.#####.#.##.##.##.##.#..###.#.......##..#.#..#.###..#...###.##....#.####..#....#.#####
    .##..#.####.#...#.....#.###..##...##.##...#..##.#......#.#####..........###.#.#.#.####.###...###.##.
    .#.####..#.#.#.....#....#.#.#.#.#..#####..##.##.#.#.#..#.#..###...##..###..#..##.####...######...###
    .#...#.####...##..#.#.####.###..#.#.#..####.###.####.##.#.#.##...#.#...#..#.##.......#....#..###...#
    "
}