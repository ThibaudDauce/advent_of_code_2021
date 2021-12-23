fn main()
{
    let burrow = Burrow {
        energy: 0,
        amphipods: [
            Amphipod { kind: Kind::Bronze, position: Position::Room(3, RoomPosition::High) },
            Amphipod { kind: Kind::Copper,  position: Position::Room(3, RoomPosition::Low) },

            Amphipod { kind: Kind::Bronze, position: Position::Room(5, RoomPosition::High) },
            Amphipod { kind: Kind::Amber, position: Position::Room(5, RoomPosition::Low) },

            Amphipod { kind: Kind::Desert, position: Position::Room(7, RoomPosition::High) },
            Amphipod { kind: Kind::Desert, position: Position::Room(7, RoomPosition::Low) },

            Amphipod { kind: Kind::Amber, position: Position::Room(9, RoomPosition::High) },
            Amphipod { kind: Kind::Copper,  position: Position::Room(9, RoomPosition::Low) },
        ]
    };

    println!("Part 1: {}", minimum_energy(&burrow))
}

#[derive(PartialEq, Clone, Copy)]
enum RoomPosition {
    High,
    Low,
}

#[derive(PartialEq, Clone)]
struct Amphipod {
    kind: Kind,
    position: Position,
}

#[derive(PartialEq, Clone, Copy)]
enum Kind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(PartialEq, Clone, Copy)]
enum Position {
    Hallway(u32), // u32 is 1..=11
    Room(u32, RoomPosition), // u32 is 3, 5, 7, 9
}

#[derive(Clone)]
struct Burrow {
    energy: u32,
    amphipods: [Amphipod; 8],
}

const HALLWAY_POSITIONS: [u32; 7] = [1, 2, 4, 6, 8, 10, 11];

fn minimum_energy(burrow: &Burrow) -> u32
{
    // print(burrow);

    if burrow_complete(burrow) {
        return burrow.energy;
    }

    let mut min_energy = u32::MAX;

    for (index, amphipod) in burrow.amphipods.iter().enumerate() {
        let x = get_x(&amphipod.position);

        let hallway_others_x: Vec<u32> = burrow.amphipods.iter().filter_map(|amphipod| {
            if let Position::Hallway(other_x) = amphipod.position {
                if other_x != x {
                    Some(other_x)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();

        let my_room_x = get_amphipod_room(&amphipod.kind);

        let first_bloc_left  = *hallway_others_x.iter().filter(|other_x| **other_x < x).max().unwrap_or(&0);
        let first_bloc_right = *hallway_others_x.iter().filter(|other_x| **other_x > x).min().unwrap_or(&12);

        match &amphipod.position {
            Position::Room(_, room_position) => {
                match room_position {
                    RoomPosition::High => {
                        // Si on est déjà dans la bonne chambre et que la personne en dessous est dans la bonne chambre
                        if x == my_room_x && get_amphipod(burrow, &Position::Room(x, RoomPosition::Low)).unwrap().kind == amphipod.kind {
                            continue;
                        }
                    },
                    RoomPosition::Low => {
                        // Si quelqu'un nous bloque…
                        if get_amphipod(burrow, &Position::Room(x, RoomPosition::High)).is_some() {
                            continue;
                        }

                        // Si on est déjà dans la bonne chambre
                        if x == my_room_x {
                            continue;
                        }
                    },
                }

                let out_energy = match room_position {
                    RoomPosition::Low => 2,
                    RoomPosition::High => 1,
                };

                if let Some(new_min_energy) = go_to_my_room(burrow, index, amphipod, x, first_bloc_left, first_bloc_right, my_room_x, out_energy) {
                    if new_min_energy < min_energy {
                        min_energy = new_min_energy;
                    }
                } else {
                    for hallway_position in HALLWAY_POSITIONS {
                        if hallway_position > x && first_bloc_right <= hallway_position {
                            continue;
                        }
                    
                        if hallway_position < x && first_bloc_left >= hallway_position {
                            continue;
                        }

                        let mut new_burrow = &mut burrow.clone();
                        new_burrow.amphipods[index].position = Position::Hallway(hallway_position);

                        new_burrow.energy += ((hallway_position as i32 - x as i32).abs() as u32 + out_energy) * get_energy(&amphipod.kind);
                        let new_min_energy = minimum_energy(new_burrow);
                        if new_min_energy < min_energy {
                            min_energy = new_min_energy;
                        }
                    }
                }
            },
            Position::Hallway(_) => {
                if let Some(new_min_energy) = go_to_my_room(burrow, index, amphipod, x, first_bloc_left, first_bloc_right, my_room_x, 0) {
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
        if let Some(amphipod) = get_amphipod(burrow, &Position::Hallway(hallway_position)) {
            print!("{}", get_amphipod_char(&amphipod.kind));
        } else {
            print!(".");
        }
    }
    println!("#");

    print!("###");
    for room_x in [3, 5, 7, 9] {
        if let Some(amphipod) = get_amphipod(burrow, &Position::Room(room_x, RoomPosition::High)) {
            print!("{}#", get_amphipod_char(&amphipod.kind));
        } else {
            print!(".#");
        }
    }
    println!("##");
    print!("  #");
    for room_x in [3, 5, 7, 9] {
        if let Some(amphipod) = get_amphipod(burrow, &Position::Room(room_x, RoomPosition::Low)) {
            print!("{}#", get_amphipod_char(&amphipod.kind));
        } else {
            print!(".#");
        }
    }
    println!();
    println!("  #########");
    println!();
}

fn go_to_my_room(burrow: &Burrow, index: usize, amphipod: &Amphipod, x: u32, first_bloc_left: u32, first_bloc_right: u32, my_room_x: u32, out_energy: u32) -> Option<u32>
{
    if get_amphipod(burrow, &Position::Room(my_room_x, RoomPosition::High)).is_some() {
        return None;
    }
    
    if my_room_x > x && first_bloc_right < my_room_x {
        return None;
    }

    if my_room_x < x && first_bloc_left > my_room_x {
        return None;
    }

    if let Some(other_amphipod) = get_amphipod(burrow, &Position::Room(my_room_x, RoomPosition::Low)) {
        if other_amphipod.kind == amphipod.kind {
            let mut new_burrow = &mut burrow.clone();
            new_burrow.amphipods[index].position = Position::Room(my_room_x, RoomPosition::High);
            new_burrow.energy += ((my_room_x as i32 - x as i32).abs() as u32 + 1 + out_energy) * get_energy(&amphipod.kind);

            let new_min_energy = minimum_energy(new_burrow);
            return Some(new_min_energy);
        }
    } else {
        let mut new_burrow = &mut burrow.clone();
        new_burrow.amphipods[index].position = Position::Room(my_room_x, RoomPosition::Low);
        new_burrow.energy += ((my_room_x as i32 - x as i32).abs() as u32 + 2 + out_energy) * get_energy(&amphipod.kind);

        let new_min_energy = minimum_energy(new_burrow);
        return Some(new_min_energy);
    }

    None
}

fn burrow_complete(burrow: &Burrow) -> bool
{
    for amphipod in &burrow.amphipods {
        match amphipod.position {
            Position::Hallway(_) => return false,
            Position::Room(x, _) => {
                if x != get_amphipod_room(&amphipod.kind) {
                    return false;
                }
            }
        }
    }

    true
}

fn get_x(position: &Position) -> u32
{
    match position {
        Position::Room(x, _) => *x,
        Position::Hallway(x) => *x,
    }
}

fn get_amphipod_room(kind: &Kind) -> u32
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

fn get_amphipod<'a>(burrow: &'a Burrow, position: &Position) -> Option<&'a Amphipod>
{
    for amphipod in &burrow.amphipods {
        if &amphipod.position == position {
            return Some(amphipod);
        }
    }

    None
}

#[test]
fn it_works()
{
    assert_eq!(12521, minimum_energy(& Burrow {
        energy: 0,
        amphipods: [
            Amphipod { kind: Kind::Bronze, position: Position::Room(3, RoomPosition::High) },
            Amphipod { kind: Kind::Amber,  position: Position::Room(3, RoomPosition::Low) },

            Amphipod { kind: Kind::Copper, position: Position::Room(5, RoomPosition::High) },
            Amphipod { kind: Kind::Desert, position: Position::Room(5, RoomPosition::Low) },

            Amphipod { kind: Kind::Bronze, position: Position::Room(7, RoomPosition::High) },
            Amphipod { kind: Kind::Copper, position: Position::Room(7, RoomPosition::Low) },

            Amphipod { kind: Kind::Desert, position: Position::Room(9, RoomPosition::High) },
            Amphipod { kind: Kind::Amber,  position: Position::Room(9, RoomPosition::Low) },
        ]
    }));
}