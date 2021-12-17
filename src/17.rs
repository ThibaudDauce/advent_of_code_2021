fn main()
{
    // println!("Part 1: {}", compute(20, 30, -10, -5).0);
    // println!("Part 2: {}", compute(20, 30, -10, -5).1);

    println!("Part 1: {}", compute(150, 193, -136, -86).0);
    println!("Part 2: {}", compute(150, 193, -136, -86).1);
}

fn compute(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> (i32, usize)
{
    assert!(min_x < max_x);
    assert!(min_y < max_y);

    assert!(min_x > 0);
    assert!(max_y < 0);

    let mut max_y_reached = i32::MIN;
    let mut velocities = vec![];

    for initial_x in 1..max_x + 1 + 5 {
        for initial_y in min_y..(min_y * -1 + 5) {

            let mut acc_x = initial_x;
            let mut acc_y = initial_y;
            let mut x = 0;
            let mut y = 0;
            let mut max_y_reached_this_time = i32::MIN;

            loop {
                x += acc_x;
                y += acc_y;

                if y > max_y_reached_this_time {
                    max_y_reached_this_time = y;
                }

                if acc_x > 0 {
                    acc_x -= 1;
                } else if acc_x < 0 {
                    acc_x += 1;
                }

                acc_y -= 1;

                if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                    velocities.push((initial_x, initial_y));
                    if max_y_reached_this_time > max_y_reached {
                        max_y_reached = max_y_reached_this_time;
                    }
                    
                    break;
                }

                if x > max_x {
                    break;
                }
                if y < min_y {
                    break;
                }
            }
        }
    }

    (max_y_reached, velocities.len())
}