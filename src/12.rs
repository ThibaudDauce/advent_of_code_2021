use std::collections::HashMap;
use std::collections::HashSet;

fn main()
{
    let result = part1(raw_input());
    println!("Part 1: {}", result);

    let result = part2(raw_input());
    println!("Part 2: {}", result);
}

fn part1(input: &'static str) -> usize
{
    let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

    for line in input.trim().lines() {
        let (x, y) = line.trim().split_once('-').unwrap();

        let entry = map.entry(x).or_insert_with(HashSet::new);
        entry.insert(y);
        let entry = map.entry(y).or_insert_with(HashSet::new);
        entry.insert(x);
    }

    let paths = compute_paths_part1(&map, vec!["start"]);

    paths.len()
}

fn compute_paths_part1(map: &HashMap<&'static str, HashSet<&'static str>>, path: Vec<&'static str>) -> Vec<Vec<&'static str>>
{
    let current_position = path.last().unwrap();

    let mut paths = vec![];

    for position in map.get(current_position).unwrap() {
        if is_lower_case(position) && path.contains(position) {
            continue;
        }

        let mut new_path = path.clone();
        new_path.push(position);

        if position == &"end" {
            paths.push(new_path);
        } else {
            let mut new_paths = compute_paths_part1(map, new_path);
            paths.append(&mut new_paths);
        }
    }

    paths
}

#[derive(Debug, Clone)]
struct Path {
    positions: Vec<&'static str>,
    double_visit: bool,
}

fn part2(input: &'static str) -> usize
{
    let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

    for line in input.trim().lines() {
        let (x, y) = line.trim().split_once('-').unwrap();

        let entry = map.entry(x).or_insert_with(HashSet::new);
        entry.insert(y);
        let entry = map.entry(y).or_insert_with(HashSet::new);
        entry.insert(x);
    }

    let path = Path { positions: vec!["start"], double_visit: false };

    let paths = compute_paths_part2(&map, path);

    paths.len()
}

fn compute_paths_part2(map: &HashMap<&'static str, HashSet<&'static str>>, path: Path) -> Vec<Path>
{
    let current_position = path.positions.last().unwrap();

    let mut paths = vec![];

    for position in map.get(current_position).unwrap() {
        if position == &"start" {
            continue;
        }

        let mut new_path = path.clone();

        if is_lower_case(position) && path.positions.contains(position) {
            if path.double_visit {
                continue;
            }

            new_path.double_visit = true;
        }

        new_path.positions.push(position);

        if position == &"end" {
            paths.push(new_path);
        } else {
            let mut new_paths = compute_paths_part2(map, new_path);
            paths.append(&mut new_paths);
        }
    }

    paths
}

#[test]
fn test_part1() {
    let result = part1("
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ");

    assert_eq!(10, result);

    let result = part1("
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ");

    assert_eq!(19, result);

    let result = part1("
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ");

    assert_eq!(226, result);
}

#[test]
fn test_part2() {
    let result = part2("
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ");

    assert_eq!(36, result);

    let result = part2("
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ");

    assert_eq!(103, result);

    let result = part2("
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ");

    assert_eq!(3509, result);
}

fn is_lower_case(position: &'static str) -> bool
{
    for char in position.chars() {
        if !char.is_ascii_lowercase() {
            return false;
        }
    }

    true
}

fn raw_input() -> &'static str
{
    "
    pq-GX
    GX-ah
    mj-PI
    ey-start
    end-PI
    YV-mj
    ah-iw
    te-GX
    te-mj
    ZM-iw
    te-PI
    ah-ZM
    ey-te
    ZM-end
    end-mj
    te-iw
    te-vc
    PI-pq
    PI-start
    pq-ey
    PI-iw
    ah-ey
    pq-iw
    pq-start
    mj-GX    
    "
}