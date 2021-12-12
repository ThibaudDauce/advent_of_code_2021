use std::collections::HashMap;
use std::collections::HashSet;

fn main()
{
    part1();
}

fn part1()
{
    let result = number_of_paths(raw_input());
    println!("Part 1: {}", result);
}

fn number_of_paths(input: &'static str) -> usize
{
    let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

    for line in input.trim().lines() {
        let (x, y) = line.trim().split_once('-').unwrap();

        let entry = map.entry(x).or_insert_with(HashSet::new);
        entry.insert(y);
        let entry = map.entry(y).or_insert_with(HashSet::new);
        entry.insert(x);
    }

    let paths = compute_paths(&map, vec!["start"]);

    dbg!(&paths);
    paths.len()
}

fn compute_paths(map: &HashMap<&'static str, HashSet<&'static str>>, path: Vec<&'static str>) -> Vec<Vec<&'static str>>
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
            let mut new_paths = compute_paths(map, new_path);
            paths.append(&mut new_paths);
        }
    }

    paths
}

#[test]
fn test_all() {
    let result = number_of_paths("
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ");

    assert_eq!(10, result);

    let result = number_of_paths("
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

    let result = number_of_paths("
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