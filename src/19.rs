use std::collections::HashSet;

fn main()
{
    let (number_of_beacons, maximum_distance) = number_of_beacons(raw_input(), 12);

    println!("Part 1: {}", number_of_beacons);
    println!("Part 2: {}", maximum_distance);
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Orientation {
    facing: Facing,
    inverse: Inverse,
    rotation: Rotation,
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    state: ScannerState,
}

#[derive(Debug)]
enum ScannerState {
    Unknown(Vec<VectorsByOrientation>),
    Known(KnownScanner),
}

#[derive(Debug)]
struct KnownScanner
{
    position: Vector,
    original_orientation: Orientation,
    vectors_by_points: Vec<PointWithVectors>,
}

#[derive(Debug, Clone)]
struct PointWithVectors {
    original_point: Vector,
    point: Vector,
    vectors: HashSet<Vector>,
}

#[derive(Debug)]
struct VectorsByOrientation {
    orientation: Orientation,
    vectors: Vec<PointWithVectors>,
}


fn number_of_beacons(input: &'static str, number_of_match: usize) -> (u32, u32)
{
    let mut scanners = vec![];

    for (id, block) in input.trim().split("\n\n").enumerate() {
        let mut beacons_original_points = vec![];

        for line in block.trim().lines().skip(1) {
            let mut elements = line.trim().split(',');

            beacons_original_points.push(Vector {
                x: elements.next().unwrap().parse().unwrap(),
                y: elements.next().unwrap().parse().unwrap(), 
                z: elements.next().unwrap().parse().unwrap(),
            });
        }

        let mut vectors: Vec<VectorsByOrientation> = vec![];
        for facing in [Facing::X, Facing::Y, Facing::Z] {
            for inverse in [Inverse::No, Inverse::Yes] {
                for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                    let new_beacons_points: Vec<(Vector, Vector)> = beacons_original_points
                        .iter()
                        .map(|point| (*point, change_coordinates(point, facing, inverse, rotation)))
                        .collect();

                    let mut vectors_for_one_orientation: Vec<PointWithVectors> = vec![];
                    for (original_point_from, point_from) in &new_beacons_points {
                        let mut vectors_for_one_point: HashSet<Vector> = HashSet::new();
                        for (_, point_to) in &new_beacons_points {
                            if point_from == point_to {
                                continue;
                            }

                            vectors_for_one_point.insert(Vector {
                                x: point_to.x - point_from.x,
                                y: point_to.y - point_from.y,
                                z: point_to.z - point_from.z,
                            });
                        }

                        let point_with_vectors = PointWithVectors {
                            original_point: *original_point_from,
                            point: *point_from,
                            vectors: vectors_for_one_point,
                        };

                        vectors_for_one_orientation.push(point_with_vectors);
                    }

                    vectors.push(VectorsByOrientation { orientation: Orientation { facing, inverse, rotation }, vectors: vectors_for_one_orientation });
                }
            }
        }

        scanners.push(Scanner { id, state: ScannerState::Unknown(vectors) });
    }

    let first_scanner = simplify_scanner(&scanners[0], &Orientation { facing: Facing::X, rotation: Rotation::R0, inverse: Inverse::No }, Vector {x: 0, y: 0, z: 0 });
    scanners[0] = first_scanner;
    // dbg!(&scanners[0]);

    loop {
        let mut scanner_modification: Option<(usize, Scanner)> = None;
        'scanner_loop: for scanner in &scanners {
            if let ScannerState::Unknown(vectors_by_orientations) = &scanner.state {
                // println!("Scanning scanner {}", scanner.id);

                for other_scanner in &scanners {
                    if let ScannerState::Known(other_scanner_state) = &other_scanner.state {
                        // println!("\t… comparing with scanner {}", other_scanner.id);
                        for other_scanner_point_with_vectors in &other_scanner_state.vectors_by_points {
                            let other_scanner_point = other_scanner_point_with_vectors.point;

                            // println!("\t\t …point {},{},{} from known scanner", other_scanner_point.x,other_scanner_point.y,other_scanner_point.z );
                            // dbg!(&other_scanner_vectors_for_one_point);
                            for vectors_by_orientation in vectors_by_orientations {
                                for point_with_vectors in &vectors_by_orientation.vectors {
                                    let point_from = point_with_vectors.point;
                                    // dbg!(&vectors);
                                    let count = point_with_vectors.vectors.intersection(&other_scanner_point_with_vectors.vectors).count();
                                    if count >= (number_of_match - 1) {
                                        scanner_modification = Some((scanner.id, simplify_scanner(scanner, &vectors_by_orientation.orientation, Vector {
                                            x: other_scanner_point.x - point_from.x,
                                            y: other_scanner_point.y - point_from.y,
                                            z: other_scanner_point.z - point_from.z,
                                        })));
                                        break 'scanner_loop;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some((index, scanner)) = scanner_modification {
            // println!("Found scanner {}", index);
            scanners[index] = scanner;
        } else {
            break;
        }
    }

    let mut points = HashSet::new();
    let mut scanner_positions = vec![];
    for scanner in &scanners {
        match &scanner.state {
            ScannerState::Unknown(_) => panic!(),
            ScannerState::Known(scanner_state) => {
                println!("Scanner {} position {},{},{}", scanner.id, scanner_state.position.x, scanner_state.position.y, scanner_state.position.z);
                scanner_positions.push(scanner_state.position);
                for point_with_vectors in &scanner_state.vectors_by_points {
                    points.insert(point_with_vectors.point);
                }
            },
        }
    }

    let mut maximum_distance = 0;
    for a in &scanner_positions {
        for b in &scanner_positions {
            let distance = manhattan_distance(a, b);
            if distance > maximum_distance {
                maximum_distance = distance;
            }
        }
    }

    (points.len() as u32, maximum_distance)
}

fn manhattan_distance(a: &Vector, b: &Vector) -> u32
{
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as u32
}

fn simplify_scanner(scanner: &Scanner, orientation: &Orientation, position: Vector) -> Scanner
{
    let scanner_vectors: Vec<PointWithVectors> = match &scanner.state {
        ScannerState::Known(_) => panic!(),
        ScannerState::Unknown(vectors_by_orientations) => {
            let mut result = None;

            for vectors_by_orientation in vectors_by_orientations {
                if vectors_by_orientation.orientation == *orientation {
                    let mut new_vectors_by_orientation = vec![];

                    for point_with_vectors in &vectors_by_orientation.vectors {
                        let point_with_vectors = PointWithVectors {
                            original_point: point_with_vectors.original_point,
                            point: Vector {
                                x: position.x + point_with_vectors.point.x,
                                y: position.y + point_with_vectors.point.y,
                                z: position.z + point_with_vectors.point.z,
                            },
                            vectors: point_with_vectors.vectors.clone(),
                        };

                        new_vectors_by_orientation.push(point_with_vectors);
                    }

                    result = Some(new_vectors_by_orientation);
                    break;
                }
            }

            result.unwrap().clone()
        },
    };
    
    Scanner { id: scanner.id, state: ScannerState::Known(KnownScanner { position, original_orientation: *orientation, vectors_by_points: scanner_vectors }) }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Facing {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Inverse {
    No,
    Yes,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

fn change_coordinates(position: &Vector, facing: Facing, inverse: Inverse, rotation: Rotation) -> Vector
{
    // let new_position = *position;

    // let new_position = if inverse == Inverse::Yes {
    //     match facing {
    //         Facing::X => Vector { x: - new_position.x, y:   new_position.y, z:   new_position.z },
    //         Facing::Y => Vector { x:   new_position.x, y: - new_position.y, z:   new_position.z },
    //         Facing::Z => Vector { x:   new_position.x, y:   new_position.y, z: - new_position.z },
    //     }
    // } else {
    //     new_position
    // };

    // let new_position = match facing {
    //     Facing::X => new_position,
    //     Facing::Y => Vector { x:   new_position.z, y: new_position.x, z: new_position.y },
    //     Facing::Z => Vector { x: - new_position.z, y: new_position.y, z: new_position.x },
    // };

    // En position normale :
    // X devant
    // Z à droite
    // Y en haut

    // À 90°
    // X devant
    // Z en haut
    // Y à gauche
    
    // À 180°
    // X devant
    // Z à gauche
    // Y en bas
    
    // À 170°
    // X devant
    // Z en bas
    // Y à droite
    
    match inverse {
        Inverse::No => {
            match facing {
                Facing::X => {
                    match rotation {
                        Rotation::R0   => *position,
                        Rotation::R90  => Vector { x: position.x, y: - position.z, z:   position.y },
                        Rotation::R180 => Vector { x: position.x, y: - position.y, z: - position.z },
                        Rotation::R270 => Vector { x: position.x, y:   position.z, z: - position.y },
                    }        
                },
                Facing::Y => {
                    match rotation {
                        Rotation::R0   => Vector { x:   position.z, y: position.x, z:   position.y },
                        Rotation::R90  => Vector { x:   position.y, y: position.x, z: - position.z },
                        Rotation::R180 => Vector { x: - position.z, y: position.x, z: - position.y },
                        Rotation::R270 => Vector { x: - position.y, y: position.x, z:   position.z },
                    }
                },
                Facing::Z => {
                    match rotation {
                        Rotation::R0   => Vector { x: - position.z, y:   position.y, z: position.x },
                        Rotation::R90  => Vector { x: - position.y, y: - position.z, z: position.x },
                        Rotation::R180 => Vector { x:   position.z, y: - position.y, z: position.x },
                        Rotation::R270 => Vector { x:   position.y, y:   position.z, z: position.x },
                    }
                },
            }
        },
        Inverse::Yes => {
            match facing {
                Facing::X => {
                    match rotation {
                        Rotation::R0   => Vector { x: - position.x, y:   position.y, z: - position.z },
                        Rotation::R90  => Vector { x: - position.x, y: - position.z, z: - position.y },
                        Rotation::R180 => Vector { x: - position.x, y: - position.y, z:   position.z },
                        Rotation::R270 => Vector { x: - position.x, y:   position.z, z:   position.y },
                    }    
                },
                Facing::Y => {
                    match rotation {
                        Rotation::R0   => Vector { x: - position.z, y: - position.x, z:   position.y },
                        Rotation::R90  => Vector { x: - position.y, y: - position.x, z: - position.z },
                        Rotation::R180 => Vector { x:   position.z, y: - position.x, z: - position.y },
                        Rotation::R270 => Vector { x:   position.y, y: - position.x, z:   position.z },
                    }
                },
                Facing::Z => {
                    match rotation {
                        Rotation::R0   => Vector { x:   position.z, y:   position.y, z: - position.x },
                        Rotation::R90  => Vector { x:   position.y, y: - position.z, z: - position.x },
                        Rotation::R180 => Vector { x: - position.z, y: - position.y, z: - position.x },
                        Rotation::R270 => Vector { x: - position.y, y:   position.z, z: - position.x },
                    }
                },
            }
        },
    }
    
    
}

fn distance(a: &Vector, b: &Vector) -> f64
{
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
}

fn v(x: i32, y: i32, z: i32) -> Vector
{
    Vector { x, y, z }
}


#[test]
fn it_works()
{
    assert_eq!(v(1, 2, 3), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::No, Rotation::R0));
    assert_eq!(v(1, -3, 2), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::No, Rotation::R90));
    assert_eq!(v(1, -2, -3), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::No, Rotation::R180));
    assert_eq!(v(1, 3, -2), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::No, Rotation::R270));

    assert_eq!(v(3, 1, 2), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::No, Rotation::R0));
    assert_eq!(v(2, 1, -3), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::No, Rotation::R90));
    assert_eq!(v(-3, 1, -2), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::No, Rotation::R180));
    assert_eq!(v(-2, 1, 3), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::No, Rotation::R270));

    assert_eq!(v(-3,  2, 1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::No, Rotation::R0));
    assert_eq!(v(-2, -3, 1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::No, Rotation::R90));
    assert_eq!(v( 3, -2, 1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::No, Rotation::R180));
    assert_eq!(v( 2,  3, 1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::No, Rotation::R270));

    assert_eq!(v(-1, 2, -3), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::Yes, Rotation::R0));
    assert_eq!(v(-1, -3, -2), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::Yes, Rotation::R90));
    assert_eq!(v(-1, -2, 3), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::Yes, Rotation::R180));
    assert_eq!(v(-1, 3, 2), change_coordinates(&v(1, 2, 3), Facing::X, Inverse::Yes, Rotation::R270));

    assert_eq!(v(-3, -1,  2), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::Yes, Rotation::R0));
    assert_eq!(v(-2, -1, -3), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::Yes, Rotation::R90));
    assert_eq!(v( 3, -1, -2), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::Yes, Rotation::R180));
    assert_eq!(v( 2, -1,  3), change_coordinates(&v(1, 2, 3), Facing::Y, Inverse::Yes, Rotation::R270));

    assert_eq!(v( 3,  2, -1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::Yes, Rotation::R0));
    assert_eq!(v( 2, -3, -1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::Yes, Rotation::R90));
    assert_eq!(v(-3, -2, -1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::Yes, Rotation::R180));
    assert_eq!(v(-2,  3, -1), change_coordinates(&v(1, 2, 3), Facing::Z, Inverse::Yes, Rotation::R270));

    let zero = Vector { x: 0, y: 0, z: 0 };
    let position = Vector { x: 1, y: 2, z: 3 };
    let base_distance = distance(&position, &zero);
    for facing in [Facing::X, Facing::Y, Facing::Z] {
        for inverse in [Inverse::No, Inverse::Yes] {
            for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                let new_distance = distance(&change_coordinates(&position, facing, inverse, rotation), &zero);
                assert_eq!(base_distance, new_distance);
            }
        }
    }

    for facing_a in [Facing::X, Facing::Y, Facing::Z] {
        for inverse_a in [Inverse::No, Inverse::Yes] {
            for rotation_a in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                for facing_b in [Facing::X, Facing::Y, Facing::Z] {
                    for inverse_b in [Inverse::No, Inverse::Yes] {
                        for rotation_b in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                            if facing_a == facing_b && inverse_a == inverse_b && rotation_a == rotation_b {
                                continue;
                            }
                            dbg!("####");
                            dbg!(facing_a, inverse_a, rotation_a);
                            dbg!(facing_b, inverse_b, rotation_b);


                            assert_ne!(change_coordinates(&position, facing_a, inverse_a, rotation_a), change_coordinates(&position, facing_b, inverse_b, rotation_b));
                        }
                    }
                }
            }
        }
    }

    let originals = vec![
        Vector { x: -1, y: -1, z: 1 },
        Vector { x: -2, y: -2, z: 2 },
        Vector { x: -3, y: -3, z: 3 },
        Vector { x: -2, y: -3, z: 1 },
        Vector { x: 5, y: 6, z: -4 },
        Vector { x: 8, y: 0, z: 7 },
    ];


    let others = vec![
        vec![
            Vector { x: 1,y: -1, z: 1},
            Vector { x: 2,y: -2, z: 2},
            Vector { x: 3, y: -3, z: 3},
            Vector { x: 2,y: -1, z: 3},
            Vector { x: -5, y: 4, z: -6},
            Vector { x: -8, y: -7, z: 0},
        ],
        vec![
            Vector { x: -1,y: -1, z: -1},
            Vector { x: -2,y: -2, z: -2},
            Vector { x: -3, y: -3, z: -3},
            Vector { x: -1,y: -3, z: -2},
            Vector { x: 4, y: 6, z: 5},
            Vector { x: -7, y: 0, z: 8},
        ],
        vec![
            Vector { x: 1,y: 1, z: -1},
            Vector { x: 2,y: 2, z: -2},
            Vector { x: 3, y: 3, z: -3},
            Vector { x: 1,y: 3, z: -2},
            Vector { x: -4, y: -6, z: 5},
            Vector { x: 7, y: 0, z: 8},
        ],
        vec![
            Vector { x: 1,y: 1, z: 1},
            Vector { x: 2,y: 2, z: 2},
            Vector { x: 3, y: 3, z: 3},
            Vector { x: 3,y: 1, z: 2},
            Vector { x: -6, y: -4, z: -5},
            Vector { x: 0, y: 7, z: -8},
        ],
    ];

    'others: for other in others {
        for facing in [Facing::X, Facing::Y, Facing::Z] {
            for inverse in [Inverse::No, Inverse::Yes] {
                for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                    let others_rotated: Vec<Vector> = other.iter().map(|point| change_coordinates(point, facing, inverse, rotation)).collect();
                    if others_rotated == originals {
                        continue 'others;
                    }
                }
            }
        }

        panic!();
    }

    assert_eq!(3, number_of_beacons(small_test_input(), 3).0);

    let (number_of_beacons, maximum_distance) = number_of_beacons(test_input(), 12);

    assert_eq!(79, number_of_beacons);
    assert_eq!(3621, maximum_distance);
}

fn all_orientations() -> [Orientation; 24]
{
    let mut orientations = [Orientation { facing : Facing::X, inverse: Inverse::No, rotation: Rotation::R0 }; 24];
    let mut i = 0;

    for facing in [Facing::X, Facing::Y, Facing::Z] {
        for inverse in [Inverse::No, Inverse::Yes] {
            for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
                orientations[i] = Orientation { facing, inverse , rotation };
                i += 1;
            }
        }
    }

    orientations
}

fn small_test_input() -> &'static str
{
    "
    --- scanner 0 ---
    0,2,0
    4,1,0
    3,3,0

    --- scanner 1 ---
    -1,-1,0
    -5,0,0
    -2,1,0
    "
}

fn test_input() -> &'static str
{
    "
    --- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401

    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390

    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562

    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596

    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    "
}


fn raw_input() -> &'static str
{
    "
    --- scanner 0 ---
    -817,-765,856
    443,-709,-511
    -658,753,-745
    378,506,-625
    557,-593,616
    -622,-827,819
    -611,-838,856
    -433,650,563
    -586,-856,-622
    398,565,499
    229,541,474
    585,-710,-578
    -584,611,490
    -796,-861,-671
    528,-778,-656
    -448,738,509
    702,-600,648
    -635,590,-725
    368,455,500
    339,605,-490
    288,624,-682
    -687,-819,-750
    -646,726,-814
    -134,-69,143
    -4,-120,3
    632,-644,504

    --- scanner 1 ---
    -576,-655,-870
    83,71,-65
    455,510,-438
    -496,-588,-822
    -601,-396,364
    -752,-444,373
    601,-737,495
    125,-92,-181
    402,514,256
    505,551,-412
    -407,683,546
    -700,501,-622
    603,-857,372
    -717,-310,415
    -409,-628,-813
    545,-770,-395
    363,354,318
    -538,654,501
    395,344,337
    466,407,-474
    -784,586,-567
    634,-809,-409
    687,-686,-444
    606,-670,479
    -593,685,436
    -752,662,-653

    --- scanner 2 ---
    -351,-447,608
    -522,-602,-725
    652,-529,515
    839,-608,-778
    116,-62,-109
    606,504,639
    -288,499,737
    -219,487,710
    -283,528,-835
    -455,-744,-726
    612,637,-413
    646,-520,584
    -408,-537,490
    -543,-498,589
    -411,427,-872
    530,514,796
    805,-675,-742
    -308,476,574
    872,621,-400
    -362,637,-877
    496,468,745
    740,489,-416
    964,-658,-827
    -377,-687,-794
    471,-547,543

    --- scanner 3 ---
    -600,-676,-662
    690,-716,598
    317,-749,-574
    716,563,-791
    444,358,756
    523,-729,495
    47,106,1
    -456,-623,-710
    -634,789,-520
    -625,659,492
    -649,745,633
    -514,796,-415
    496,487,728
    340,-665,-745
    -548,-726,-667
    762,490,-879
    -64,40,-123
    -576,-661,725
    867,555,-818
    346,499,705
    559,-583,601
    305,-766,-626
    -584,685,482
    -652,-454,737
    -455,845,-563
    -612,-636,677

    --- scanner 4 ---
    555,657,454
    -681,394,490
    -492,305,-497
    -707,-820,611
    720,-949,377
    818,447,-591
    -631,473,537
    770,492,-727
    193,7,-89
    -246,-812,-593
    -466,321,-586
    696,-780,402
    637,-890,-779
    -659,-811,762
    -793,-769,727
    467,577,391
    755,-800,-722
    -366,-689,-591
    774,355,-587
    656,-839,-587
    -433,-705,-596
    806,-810,403
    417,625,515
    109,-153,6
    -671,504,536
    -457,253,-428

    --- scanner 5 ---
    492,549,-889
    676,-597,352
    -552,-915,660
    685,-637,439
    -521,633,-789
    -2,-104,3
    -431,515,-846
    -102,40,-105
    515,386,-909
    503,413,-978
    290,489,399
    733,-570,519
    -541,574,335
    441,528,326
    -511,684,336
    272,-416,-691
    305,401,345
    -501,-829,544
    307,-539,-744
    258,-455,-602
    -482,534,-654
    -510,-781,695
    -632,-488,-658
    -616,-555,-663
    -509,-368,-660
    -372,598,310

    --- scanner 6 ---
    514,-446,-382
    414,762,-372
    -307,640,368
    -763,741,-861
    -499,-612,393
    415,-376,-468
    -304,643,449
    -518,-643,425
    381,743,-590
    795,-343,698
    -592,-701,-409
    -824,855,-903
    838,-421,652
    -429,629,495
    430,-425,-453
    858,-439,598
    584,691,680
    448,619,744
    -608,-645,-578
    -416,-656,379
    52,-36,-100
    553,722,676
    -37,110,83
    -842,751,-763
    371,655,-417
    -529,-773,-524

    --- scanner 7 ---
    -687,452,941
    -356,789,-401
    811,-554,-481
    897,-543,-349
    594,-328,503
    624,844,676
    -580,503,809
    638,-375,558
    -25,138,155
    832,839,-318
    748,890,-467
    -609,402,922
    -430,859,-504
    802,-497,-426
    -521,-476,669
    -360,-391,667
    -604,-522,-809
    -505,-560,-728
    -381,795,-329
    98,-36,82
    590,706,799
    -740,-513,-735
    -434,-514,604
    608,919,748
    861,804,-541
    689,-335,673

    --- scanner 8 ---
    -396,443,652
    -446,501,582
    883,-477,-273
    893,965,-608
    823,-389,-359
    938,922,-715
    42,-10,-17
    -669,-725,483
    -701,-787,591
    -373,-699,-453
    -744,549,-479
    928,822,493
    -472,490,571
    688,-496,-328
    618,-742,726
    152,149,23
    -556,-654,-489
    684,-778,544
    -650,-739,456
    720,955,-696
    -392,-555,-479
    674,-732,650
    842,921,423
    -665,709,-497
    -676,599,-652
    857,906,472

    --- scanner 9 ---
    -74,11,-72
    -84,-133,86
    668,503,-564
    539,815,778
    522,656,693
    -667,720,-632
    453,-604,677
    677,497,-633
    -711,-836,795
    -493,463,605
    -581,710,-619
    -806,-655,798
    699,-692,-638
    -495,-692,-520
    750,-500,-657
    589,690,729
    510,-526,745
    650,322,-592
    -671,712,-682
    -488,524,641
    595,-671,676
    660,-557,-539
    -587,-688,-476
    -455,323,626
    -632,-552,-524
    -695,-710,809

    --- scanner 10 ---
    -414,391,538
    -875,-448,-650
    373,308,473
    -949,-584,-641
    654,-692,573
    -426,393,596
    529,-667,-687
    -446,273,587
    527,-828,-772
    -79,-99,-74
    640,-697,443
    5,43,50
    709,558,-396
    505,-743,531
    -625,-713,705
    -830,575,-716
    434,346,377
    -636,-849,854
    -536,-746,797
    445,383,382
    -830,494,-503
    -875,519,-578
    464,-754,-649
    -868,-683,-603
    748,531,-351
    801,499,-520

    --- scanner 11 ---
    421,-382,-811
    454,-471,-719
    -823,-412,652
    413,618,635
    -615,517,821
    475,808,-587
    -659,572,774
    -599,-532,-347
    583,-795,627
    500,683,673
    -918,-335,606
    484,-714,589
    -832,-467,595
    424,659,-545
    -710,687,814
    -641,664,-871
    -583,665,-782
    -535,632,-812
    719,-721,636
    318,-482,-868
    -461,-512,-436
    9,143,51
    470,794,-457
    -122,11,-24
    -384,-464,-361
    339,681,758

    --- scanner 12 ---
    677,-807,-475
    -514,-719,780
    -723,-601,-559
    840,468,-281
    -721,-664,-469
    682,477,-349
    -720,670,653
    100,12,-28
    541,-759,-520
    -448,443,-736
    -758,682,618
    -333,-691,762
    33,-147,98
    489,-626,862
    677,377,661
    -508,-650,789
    546,-744,772
    675,257,784
    -757,803,548
    -731,-651,-555
    -262,475,-694
    681,-645,811
    681,-763,-383
    672,444,677
    771,503,-333
    -426,499,-674

    --- scanner 13 ---
    -473,-566,-801
    520,720,876
    -523,489,329
    -725,-624,671
    -687,593,327
    830,-456,-396
    -423,-636,-859
    705,-676,642
    901,-477,-480
    -499,-555,-812
    -474,559,369
    -703,668,-703
    380,824,858
    693,712,-347
    157,-9,-102
    -771,495,-753
    911,731,-380
    422,710,797
    -764,-781,582
    759,-804,563
    -791,-790,654
    -692,513,-593
    815,-407,-381
    866,624,-337
    874,-732,626
    -4,-74,33

    --- scanner 14 ---
    -534,368,659
    -788,-761,-491
    -878,-818,-609
    -464,512,615
    25,-4,112
    367,-626,-472
    -723,-647,724
    -581,-714,725
    589,352,-779
    387,739,618
    567,-608,857
    -609,480,608
    678,-663,846
    -638,-792,793
    583,-482,843
    420,790,648
    -158,-135,147
    -768,363,-550
    669,415,-691
    -113,-39,-22
    -798,364,-440
    -827,-626,-590
    504,-766,-476
    300,748,687
    -937,338,-526
    637,476,-829
    459,-630,-428

    --- scanner 15 ---
    -601,-717,591
    471,744,-416
    633,-619,-476
    -856,501,412
    -6,160,45
    -762,-665,560
    628,-610,-709
    -753,490,376
    -712,540,350
    -146,23,18
    -580,-372,-608
    406,-672,715
    -609,-503,-563
    -919,430,-399
    620,-737,707
    -578,-558,-498
    511,-717,723
    471,610,828
    -677,-749,505
    -749,437,-318
    445,842,-377
    443,406,874
    655,-563,-506
    -909,398,-380
    497,717,-299
    384,443,802

    --- scanner 16 ---
    -801,655,-363
    482,-522,-398
    602,-354,555
    -594,-657,-859
    -511,864,386
    54,93,4
    705,-446,598
    -789,690,-401
    -866,-353,702
    600,817,-607
    655,-498,603
    -104,15,104
    -810,-532,745
    728,419,691
    532,-518,-390
    -922,585,-364
    531,826,-613
    572,799,-788
    545,-432,-400
    -584,818,474
    644,442,520
    -618,-637,-695
    -817,-412,850
    -576,969,447
    -562,-699,-673
    745,407,639

    --- scanner 17 ---
    733,-307,-527
    -583,868,-693
    667,797,-426
    70,174,-81
    -450,-610,-736
    610,-626,389
    816,881,-477
    -517,909,-589
    -346,864,714
    593,910,-512
    805,-332,-521
    816,705,594
    819,479,574
    -716,-406,591
    -641,-619,-639
    -568,-679,-796
    704,-660,367
    740,-636,400
    -398,830,724
    -554,879,-786
    -745,-392,488
    -508,808,760
    787,639,555
    630,-381,-457
    -699,-394,397
    -23,29,4

    --- scanner 18 ---
    -609,-497,-861
    -749,-667,561
    -719,-467,-860
    -524,334,-797
    -563,485,-870
    562,720,-844
    -436,349,-851
    -508,414,513
    535,777,-677
    399,582,254
    368,-800,319
    -698,-880,543
    476,536,349
    531,593,340
    -626,-805,527
    591,-765,290
    -74,79,4
    794,-457,-652
    -623,374,462
    517,-851,403
    -453,367,557
    880,-508,-536
    -633,-328,-886
    746,-466,-501
    602,729,-757
    75,-21,-160

    --- scanner 19 ---
    -817,-615,-560
    -807,587,607
    -639,-366,727
    549,-811,-382
    -663,635,554
    22,94,40
    610,495,-385
    849,-559,685
    -717,866,-533
    -855,-755,-581
    470,-784,-333
    -715,630,504
    686,-483,658
    -939,821,-534
    436,-724,-339
    -729,-368,664
    360,479,606
    491,471,758
    -118,21,-56
    -643,-358,677
    744,531,-309
    -814,950,-496
    -822,-554,-596
    723,423,-403
    797,-516,689
    367,603,722

    --- scanner 20 ---
    -707,534,-290
    -488,377,466
    -491,574,510
    859,-343,-465
    615,-893,839
    -664,-864,-409
    -454,-687,819
    -560,-651,673
    447,597,-654
    526,580,-481
    920,-448,-400
    -762,583,-427
    480,415,695
    597,557,-563
    466,474,537
    -786,524,-307
    34,-31,22
    -527,495,571
    -557,-520,806
    -687,-714,-397
    786,-870,755
    -715,-766,-268
    832,-494,-473
    667,-905,856
    482,423,421

    --- scanner 21 ---
    438,-512,-856
    562,-627,-808
    516,577,-789
    -549,-597,868
    43,5,13
    646,636,558
    -535,-644,-343
    -681,-729,884
    184,-95,123
    474,-636,449
    713,494,559
    -477,-560,-368
    -677,576,-484
    -614,567,542
    -730,555,-555
    602,-535,412
    502,-709,-825
    -421,-745,871
    -648,500,506
    -540,-518,-418
    553,-745,407
    567,594,-761
    568,579,490
    -773,550,457
    -738,684,-506
    559,500,-844

    --- scanner 22 ---
    139,7,36
    597,513,782
    846,-774,604
    729,-705,-721
    -554,-568,688
    -265,406,-834
    -274,-414,-697
    -521,706,469
    -576,717,580
    488,292,-471
    658,549,787
    -702,703,564
    -418,-616,643
    -409,-404,-797
    -7,-172,97
    105,-155,-83
    401,335,-584
    696,-736,-832
    428,286,-505
    653,-732,-884
    844,-711,694
    854,-724,608
    457,534,820
    -558,-612,780
    -388,520,-774
    -413,-528,-688
    -473,413,-807

    --- scanner 23 ---
    -470,462,-472
    397,-623,373
    874,565,-809
    -577,597,-441
    825,428,359
    807,-769,-685
    -558,402,-386
    -859,601,431
    -401,-666,-586
    702,-701,-788
    840,443,310
    -813,611,537
    -333,-687,-584
    -68,22,-17
    -428,-543,-525
    84,-16,-148
    460,-723,277
    -606,-747,663
    -823,667,613
    -610,-678,801
    671,-705,-757
    841,347,-828
    -581,-728,769
    741,357,380
    417,-677,355
    884,392,-721

    --- scanner 24 ---
    -583,-471,-740
    695,458,-641
    -464,566,-502
    363,-388,460
    437,-318,563
    682,528,583
    -601,-258,-689
    587,-674,-750
    732,500,-773
    616,-775,-863
    -854,-373,600
    388,-382,388
    -378,582,-453
    -741,-287,526
    -394,482,-547
    -617,-333,-617
    -825,-338,377
    47,144,1
    -583,694,406
    -423,713,374
    771,572,-697
    -649,733,377
    694,-725,-882
    -119,51,-90
    692,539,453
    716,506,489

    --- scanner 25 ---
    825,-589,-478
    -5,-111,11
    -756,-672,559
    -481,-363,-407
    -41,69,169
    752,-804,598
    -686,849,-763
    791,651,-479
    -463,-418,-541
    -622,-677,458
    642,-794,644
    654,647,-619
    -786,-662,525
    841,-615,-389
    -459,735,657
    -496,796,553
    489,432,562
    -786,859,-628
    -444,859,710
    -677,735,-624
    893,-580,-320
    673,-754,645
    569,344,499
    699,695,-483
    -463,-351,-577
    521,294,623

    --- scanner 26 ---
    -795,-443,-559
    -804,-231,881
    447,-796,-426
    787,552,772
    323,-746,-474
    -939,-241,781
    -692,879,-385
    -800,-355,772
    534,-231,539
    761,681,-643
    -740,586,891
    -655,-439,-696
    -948,579,918
    719,720,-763
    481,-355,554
    -109,74,78
    725,489,700
    386,-658,-394
    377,-294,591
    710,651,-676
    -637,890,-496
    -659,-395,-604
    -511,876,-480
    -825,693,858
    736,637,788

    --- scanner 27 ---
    649,-535,687
    -453,717,-503
    -656,483,676
    -709,-760,-314
    -416,655,-698
    30,-136,63
    550,-661,-354
    -96,6,-70
    -798,-651,348
    -773,-675,517
    615,-658,825
    -624,490,821
    645,-831,-347
    -768,534,751
    631,-586,900
    -782,-721,-465
    -416,728,-492
    601,-776,-394
    -772,-565,487
    625,541,-560
    758,567,725
    -763,-848,-329
    810,450,844
    796,555,858
    543,479,-582
    565,396,-625

    --- scanner 28 ---
    -730,770,-606
    -308,-494,617
    716,721,-503
    938,683,335
    -735,560,-557
    550,-782,-727
    -802,-262,-939
    -477,-478,561
    494,-587,528
    -693,669,-478
    852,710,-414
    135,108,-167
    887,729,301
    -662,-244,-835
    -423,-557,689
    480,-670,572
    879,698,-483
    -760,-323,-825
    -271,560,417
    596,-657,-713
    781,693,411
    469,-779,-724
    526,-568,559
    -258,576,677
    33,-21,-16
    -274,708,545

    --- scanner 29 ---
    -730,-682,812
    582,-864,-690
    436,772,-425
    644,-964,-631
    -719,588,-416
    486,-851,510
    -871,-700,686
    -766,-590,650
    -803,643,-501
    -487,-549,-382
    526,815,855
    -68,-180,-10
    705,-954,-635
    -515,388,673
    615,801,894
    -428,397,713
    668,766,771
    73,-15,108
    477,-750,560
    447,736,-322
    -490,-581,-412
    -595,340,740
    461,-900,431
    -771,557,-577
    -429,-484,-442
    486,798,-466

    --- scanner 30 ---
    -676,-823,-346
    -635,-849,-396
    -685,-639,809
    520,-325,-262
    -411,813,-492
    483,-484,816
    -524,-608,861
    -470,913,-554
    -719,498,577
    548,-371,-466
    -466,903,-615
    697,-492,881
    -600,-694,893
    407,740,-578
    -779,-787,-357
    569,774,-569
    88,28,-12
    588,-476,861
    520,692,592
    461,-397,-299
    -659,433,519
    -689,530,580
    572,819,705
    611,788,-568
    472,733,727
    -64,-55,137

    --- scanner 31 ---
    -723,642,402
    -674,-706,617
    600,459,590
    737,-568,299
    585,426,791
    -550,618,-599
    -561,-322,-516
    688,-618,424
    -603,474,-551
    -633,-679,792
    361,-481,-822
    -477,-299,-687
    327,-643,-921
    537,564,-830
    538,342,-785
    -500,566,-594
    -62,44,-15
    652,-551,458
    -460,-287,-635
    -660,552,404
    500,467,-914
    -605,-697,611
    -673,638,394
    271,-657,-843
    564,464,651

    --- scanner 32 ---
    677,-408,-808
    -395,369,546
    627,-359,-933
    622,653,-890
    583,859,407
    601,582,-837
    -623,-342,718
    610,-444,-774
    -607,-328,592
    -648,771,-940
    -718,-339,537
    541,-460,486
    -490,657,-922
    677,942,399
    -401,501,531
    -405,-532,-581
    -400,-496,-642
    683,-461,553
    600,887,356
    -367,-511,-441
    598,-392,408
    493,604,-898
    -432,436,684
    -18,89,12
    -580,632,-900
    "
}

