use std::cmp::min;

fn main()
{
    println!("Part 1: {}", part1(10, 6));
}

fn part1(mut player_1_position: usize, mut player_2_position: usize) -> usize
{
    let dices = &mut (1..=100).cycle().enumerate().map(|(index, dice)| (index + 1, dice));

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let (player_1, player_2, rolls) = loop {
        let (_, a) = dices.next().unwrap();
        println!("… roll {}", a);
        let (_, b) = dices.next().unwrap();
        println!("… roll {}", b);
        let (rolls, c) = dices.next().unwrap();
        println!("… roll {}", c);

        player_1_position += a + b + c;
        player_1_position %= 10;
        if player_1_position == 0 {
            player_1_position = 10;
        }

        player_1_score += player_1_position;
        println!("… player 1 in {} with score {}", player_1_position, player_1_score);

        if player_1_score >= 1000 {
            break (player_1_score, player_2_score, rolls);
        }

        let (_, a) = dices.next().unwrap();
        println!("… roll {}", a);
        let (_, b) = dices.next().unwrap();
        println!("… roll {}", b);
        let (rolls, c) = dices.next().unwrap();
        println!("… roll {}", c);

        player_2_position += a + b + c;
        player_2_position %= 10;
        if player_2_position == 0 {
            player_2_position = 10;
        }

        player_2_score += player_2_position;
        println!("… player 2 in {} with score {}", player_2_position, player_2_score);

        if player_2_score >= 1000 {
            break (player_1_score, player_2_score, rolls);
        }
    };
    
    min(player_1, player_2) * rolls
}

#[test]
fn it_works()
{
    assert_eq!(739785, part1(4, 8));
}