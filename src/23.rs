fn main()
{
    println!("Part 1: {}", minimum_energy(u32::MAX, &input_part1()));
    println!("Part 2: {}", minimum_energy(u32::MAX, &input_part2()));
}

#[derive(PartialEq, Clone, Copy)]
struct Amphipod {
    id: u8,
    kind: Kind,
}

#[derive(PartialEq, Clone, Copy)]
enum Kind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Position {
    Hallway(u8), // u8 is 1..=11
    Room(u8, u8), // first is 3, 5, 7, 9, second 1, 2, 3, 4
}

impl Position {
    fn hash(&self) -> usize
    {
        match self {
            Position::Hallway(x) => *x as usize,
            Position::Room(x, room_position) => 12 + *x as usize * 4 + *room_position as usize - 1
        }
    }
}

#[derive(Clone)]
struct Burrow {
    energy: u32,
    amphipods_by_positions: Map,
    max_depth: u8,
}

#[derive(Clone)]
struct Map {
    positions: [Option<(Position, Amphipod)>; 65]
}

impl Map {
    fn from(amphipods: &[(Position, Amphipod)]) -> Map
    {
        let mut positions = [None; 65];

        for (position, amphipod) in amphipods {
            positions[position.hash()] = Some((*position, *amphipod));
        }

        Map { positions }
    }

    fn to_vec(&self) -> (Vec<(Position, Amphipod)>, Vec<Block>)
    {
        let mut result = Vec::with_capacity(8);
        let mut blocks = Vec::with_capacity(8);

        for value in &self.positions {
            if let Some((position, amphipod)) = value {
                result.push((*position, *amphipod));
                if let Position::Hallway(x) = position {
                    blocks.push(Block { x: *x, amphipod_id: Some((amphipod.id, get_amphipod_room(&amphipod.kind))) })
                }
            }
        }

        (result, blocks)
    }

    fn get(&self, position: &Position) -> Option<Amphipod>
    {
        self.positions[position.hash()].map(|(_, amphipod)| amphipod)
    }

    fn contains_key(&self, position: &Position) -> bool
    {
        self.positions[position.hash()].is_some()
    }

    fn remove(&mut self, position: &Position)
    {
        self.positions[position.hash()] = None;
    }

    fn insert(&mut self, position: &Position, amphipod: Amphipod)
    {
        self.positions[position.hash()] = Some((*position, amphipod));
    }
}

const HALLWAY_POSITIONS: [u8; 7] = [1, 2, 4, 6, 8, 10, 11];

fn minimum_energy(mut min_energy: u32, burrow: &Burrow) -> u32
{
    // print(burrow);

    let (amphipods, hallway_blocks) = burrow.amphipods_by_positions.to_vec();


    if burrow_complete(&amphipods) {
        return burrow.energy;
    }

    if burrow.energy >= min_energy {
        return u32::MAX;
    }

    // let mut amphipods_by_positions = HashMap::with_capacity(burrow.amphipods.len());
    // for amphipod in &burrow.amphipods {
    //     amphipods_by_positions.insert(amphipod.position, amphipod);
    // }


    // let mut blocked_by: HashMap<u32, HashSet<u32>> = HashMap::with_capacity(burrow.amphipods.len());

    // for amphipod in &hallway_blocks {
    //     let (amphipod_id, amphipod_room) = amphipod.amphipod_id.unwrap();
    //     let entry = blocked_by.entry(amphipod_id).or_insert_with(|| HashSet::with_capacity(burrow.amphipods.len()));

    //     for other_amphipod in &hallway_blocks {
    //         let (other_amphipod_id, _) = other_amphipod.amphipod_id.unwrap();
    //         if other_amphipod_id == amphipod_id {
    //             continue;
    //         }

    //         // Si je suis à gauche de ma chambre
    //         if amphipod.x < amphipod_room && other_amphipod.x > amphipod.x && other_amphipod.x < amphipod_room {
    //             entry.insert(other_amphipod_id);
    //         }

    //         // Si je suis à droite de ma chambre
    //         if amphipod.x > amphipod_room && other_amphipod.x < amphipod.x && other_amphipod.x > amphipod_room {
    //             entry.insert(other_amphipod_id);
    //         }
    //     }
    // }

    'amphipod_loop: for (position, amphipod) in &amphipods {
        // if let Some(blocking_amphipods_ids) = blocked_by.get(&amphipod.id) {
        //     for blocking_amphipod_id in blocking_amphipods_ids {
        //         if blocked_by.get(blocking_amphipod_id).unwrap().contains(&amphipod.id) {
        //             return u32::MAX;
        //         }
        //     }
        // }

        let x = get_x(position);

        let my_room_x = get_amphipod_room(&amphipod.kind);

        let first_bloc_left: Block  = *hallway_blocks.iter().filter(|block| block.x < x).max_by(|a, b| a.x.cmp(&b.x)).unwrap_or(&Block { x: 0, amphipod_id: None });
        let first_bloc_right: Block = *hallway_blocks.iter().filter(|block| block.x > x).min_by(|a, b| a.x.cmp(&b.x)).unwrap_or(&Block { x: 12, amphipod_id: None });

        match position {
            Position::Room(_, room_position) => {
                if x == my_room_x {
                    let mut can_stay = true;
                    for other_room_position in (room_position + 1)..=burrow.max_depth {
                        if burrow.amphipods_by_positions.get(&Position::Room(x, other_room_position)).unwrap().kind != amphipod.kind {
                            can_stay = false;
                            break;
                        }
                    }

                    if can_stay {
                        continue 'amphipod_loop;
                    }
                }

                for other_room_position in 1..*room_position {
                    if burrow.amphipods_by_positions.contains_key(&Position::Room(x, other_room_position)) {
                        continue 'amphipod_loop;
                    }
                }

                if let Some(new_min_energy) = go_to_my_room(min_energy, burrow, position, amphipod, x, first_bloc_left, first_bloc_right, my_room_x, *room_position) {
                    if new_min_energy < min_energy {
                        min_energy = new_min_energy;
                    }
                } else {
                    for hallway_position in HALLWAY_POSITIONS {
                        if hallway_position > x && first_bloc_right.x <= hallway_position {
                            continue;
                        }
                    
                        if hallway_position < x && first_bloc_left.x >= hallway_position {
                            continue;
                        }

                        let mut new_burrow = &mut burrow.clone();
                        new_burrow.amphipods_by_positions.remove(position);
                        new_burrow.amphipods_by_positions.insert(&Position::Hallway(hallway_position), *amphipod);
            
                        new_burrow.energy += ((hallway_position as i32 - x as i32).abs() as u32 + *room_position as u32) * get_energy(&amphipod.kind);
                        let new_min_energy = minimum_energy(min_energy, new_burrow);
                        if new_min_energy < min_energy {
                            min_energy = new_min_energy;
                        }
                    }
                }
            },
            Position::Hallway(_) => {
                // check dead locks


                if let Some(new_min_energy) = go_to_my_room(min_energy, burrow, position, amphipod, x, first_bloc_left, first_bloc_right, my_room_x, 0) {
                    if new_min_energy < min_energy {
                        min_energy = new_min_energy;
                    }
                }
            },
        }
    }

    min_energy
}

fn print(burrow: &Burrow) 
{
    println!("#############");
    print!("#");
    for hallway_position in 1..=11 {
        if let Some(amphipod) = burrow.amphipods_by_positions.get(&Position::Hallway(hallway_position)) {
            print!("{}", get_amphipod_char(&amphipod.kind));
        } else {
            print!(".");
        }
    }
    println!("#");

    for room_position in 1..=burrow.max_depth {
        print!("###");
        for room_x in [3, 5, 7, 9] {
            if let Some(amphipod) = burrow.amphipods_by_positions.get(&Position::Room(room_x, room_position)) {
                print!("{}#", get_amphipod_char(&amphipod.kind));
            } else {
                print!(".#");
            }
        }
        println!("##");
    }
    println!("  #########");
    println!();
}

#[derive(Clone, Copy)]
struct Block {
    x: u8,
    amphipod_id: Option<(u8, u8)> 
}

fn go_to_my_room(min_energy: u32, burrow: &Burrow, position: &Position, amphipod: &Amphipod, x: u8, first_bloc_left: Block, first_bloc_right: Block, my_room_x: u8, out_energy: u8) -> Option<u32>
{
    if my_room_x > x && first_bloc_right.x < my_room_x {
        return None;
    }

    if my_room_x < x && first_bloc_left.x > my_room_x {
        return None;
    }

    for other_room_position in (1..=burrow.max_depth).rev() {
        if let Some(other_amphipod) = burrow.amphipods_by_positions.get(&Position::Room(my_room_x, other_room_position)) {
            if other_amphipod.kind != amphipod.kind {
                return None;
            }
        } else {
            let mut new_burrow = &mut burrow.clone();
            new_burrow.amphipods_by_positions.remove(position);
            new_burrow.amphipods_by_positions.insert(&Position::Room(my_room_x, other_room_position), *amphipod);
            new_burrow.energy += ((my_room_x as i32 - x as i32).abs() as u32 + other_room_position as u32 + out_energy as u32) * get_energy(&amphipod.kind);

            let new_min_energy = minimum_energy(min_energy, new_burrow);
            return Some(new_min_energy);
        }
    }

    print(burrow);
    panic!();
}

fn energy_as_the_crow_flies(amphipods: &Vec<(Position, Amphipod)>) -> u32
{
    let mut energy = 0;

    for (position, amphipod) in amphipods {
        let x = get_x(&position);
        let room_x = get_amphipod_room(&amphipod.kind);

        energy += (room_x as i32 - x as i32).abs() as u32;
    }

    energy
}

fn burrow_complete(amphipods: &Vec<(Position, Amphipod)>) -> bool
{
    for (position, amphipod) in amphipods {
        match position {
            Position::Hallway(_) => return false,
            Position::Room(x, _) => {
                if *x != get_amphipod_room(&amphipod.kind) {
                    return false;
                }
            }
        }
    }

    true
}

fn get_x(position: &Position) -> u8
{
    match position {
        Position::Room(x, _) => *x,
        Position::Hallway(x) => *x,
    }
}

fn get_amphipod_room(kind: &Kind) -> u8
{
    match kind {
        Kind::Amber => 3,
        Kind::Bronze => 5,
        Kind::Copper => 7,
        Kind::Desert => 9,
    }
}

fn get_amphipod_char(kind: &Kind) -> &'static str
{
    match kind {
        Kind::Amber => "A",
        Kind::Bronze => "B",
        Kind::Copper => "C",
        Kind::Desert => "D",
    }
}

fn get_energy(kind: &Kind) -> u32
{
    match kind {
        Kind::Amber => 1,
        Kind::Bronze => 10,
        Kind::Copper => 100,
        Kind::Desert => 1000,
    }
}

#[test]
fn it_works()
{
    assert_eq!(12521, minimum_energy(u32::MAX, & Burrow {
        energy: 0,
        max_depth: 2,
        amphipods_by_positions: Map::from(&[
            (Position::Room(3, 1), Amphipod { id: 0, kind: Kind::Bronze }),
            (Position::Room(3, 2), Amphipod { id: 1, kind: Kind::Amber }),

            (Position::Room(5, 1), Amphipod { id: 2, kind: Kind::Copper }),
            (Position::Room(5, 2), Amphipod { id: 3, kind: Kind::Desert }),

            (Position::Room(7, 1), Amphipod { id: 4, kind: Kind::Bronze }),
            (Position::Room(7, 2), Amphipod { id: 5, kind: Kind::Copper }),

            (Position::Room(9, 1), Amphipod { id: 6, kind: Kind::Desert }),
            (Position::Room(9, 2), Amphipod { id: 7, kind: Kind::Amber }),
        ])
    }));

    assert_eq!(44169, minimum_energy(u32::MAX, & Burrow {
        energy: 0,
        max_depth: 4,
        amphipods_by_positions: Map::from(&[
            (Position::Room(3, 1), Amphipod { id: 0, kind: Kind::Bronze }),
            (Position::Room(3, 2), Amphipod { id: 0, kind: Kind::Desert }),
            (Position::Room(3, 3), Amphipod { id: 0, kind: Kind::Desert }),
            (Position::Room(3, 4), Amphipod { id: 1, kind: Kind::Amber }),

            (Position::Room(5, 1), Amphipod { id: 2, kind: Kind::Copper }),
            (Position::Room(5, 2), Amphipod { id: 2, kind: Kind::Copper }),
            (Position::Room(5, 3), Amphipod { id: 2, kind: Kind::Bronze }),
            (Position::Room(5, 4), Amphipod { id: 3, kind: Kind::Desert }),

            (Position::Room(7, 1), Amphipod { id: 4, kind: Kind::Bronze }),
            (Position::Room(7, 2), Amphipod { id: 4, kind: Kind::Bronze }),
            (Position::Room(7, 3), Amphipod { id: 4, kind: Kind::Amber }),
            (Position::Room(7, 4), Amphipod { id: 5, kind: Kind::Copper }),

            (Position::Room(9, 1), Amphipod { id: 6, kind: Kind::Desert }),
            (Position::Room(9, 2), Amphipod { id: 6, kind: Kind::Amber }),
            (Position::Room(9, 3), Amphipod { id: 6, kind: Kind::Copper }),
            (Position::Room(9, 4), Amphipod { id: 7, kind: Kind::Amber }),
        ])
    }));

    // assert_eq!(11608, minimum_energy(u32::MAX, &input_part1()));
}

fn input_part1() -> Burrow
{
    Burrow {
        energy: 0,
        max_depth: 2,
        amphipods_by_positions: Map::from(&[
            (Position::Room(3, 1), Amphipod { id: 1, kind: Kind::Bronze }),
            (Position::Room(3, 2), Amphipod { id: 2, kind: Kind::Copper }),

            (Position::Room(5, 1), Amphipod { id: 3, kind: Kind::Bronze }),
            (Position::Room(5, 2), Amphipod { id: 4, kind: Kind::Amber }),

            (Position::Room(7, 1), Amphipod { id: 5, kind: Kind::Desert }),
            (Position::Room(7, 2), Amphipod { id: 6, kind: Kind::Desert }),

            (Position::Room(9, 1), Amphipod { id: 7, kind: Kind::Amber }),
            (Position::Room(9, 2), Amphipod { id: 8, kind: Kind::Copper }),
        ])
    }
}

fn input_part2() -> Burrow
{
    Burrow {
        energy: 0,
        max_depth: 4,
        amphipods_by_positions: Map::from(&[
            (Position::Room(3, 1), Amphipod { id: 1, kind: Kind::Bronze }),
            (Position::Room(3, 2), Amphipod { id: 0, kind: Kind::Desert }),
            (Position::Room(3, 3), Amphipod { id: 0, kind: Kind::Desert }),
            (Position::Room(3, 4), Amphipod { id: 2, kind: Kind::Copper }),

            (Position::Room(5, 1), Amphipod { id: 3, kind: Kind::Bronze }),
            (Position::Room(5, 2), Amphipod { id: 2, kind: Kind::Copper }),
            (Position::Room(5, 3), Amphipod { id: 2, kind: Kind::Bronze }),
            (Position::Room(5, 4), Amphipod { id: 4, kind: Kind::Amber }),

            (Position::Room(7, 1), Amphipod { id: 5, kind: Kind::Desert }),
            (Position::Room(7, 2), Amphipod { id: 4, kind: Kind::Bronze }),
            (Position::Room(7, 3), Amphipod { id: 4, kind: Kind::Amber }),
            (Position::Room(7, 4), Amphipod { id: 6, kind: Kind::Desert }),

            (Position::Room(9, 1), Amphipod { id: 7, kind: Kind::Amber }),
            (Position::Room(9, 2), Amphipod { id: 6, kind: Kind::Amber }),
            (Position::Room(9, 3), Amphipod { id: 6, kind: Kind::Copper }),
            (Position::Room(9, 4), Amphipod { id: 8, kind: Kind::Copper }),
        ])
    }
}