use std::cmp::min;
use std::cmp::max;

const DICES: [(usize, usize); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn main()
{
    println!("Part 1: {}", part1(10, 6));
    println!("Part 2: {}", part2(10, 6));
}

#[derive(Clone)]
struct Player {
    id: usize,
    position: usize,
    score: usize,
    turn: usize,
    win_condition: usize,
}

impl Player {
    fn play(&mut self, dice: usize) -> bool
    {
        let mut position = self.position + dice;
        position %= 10;
        if position == 0 {
            position = 10;
        }

        self.position = position;
        self.score += position;
        self.turn += 1;

        self.score >= self.win_condition
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Player1,
    Player2,
}

fn part2(player_1_position: usize, player_2_position: usize) -> usize
{
    let player_1 = Player { id: 1, position: player_1_position, score: 0, turn: 0, win_condition: 21 };
    let player_2 = Player { id: 2, position: player_2_position, score: 0, turn: 0, win_condition: 21 };

    let (player_1, player_2) = compute(player_1, player_2, Turn::Player1);

    max(player_1, player_2)
}

fn compute(player_1: Player, player_2: Player, turn: Turn) -> (usize, usize)
{
    let next_turn = match turn {
        Turn::Player1 => Turn::Player2,
        Turn::Player2 => Turn::Player1,
    };

    let mut total_player_1_wins = 0;
    let mut total_player_2_wins = 0;

    for (dice, times) in DICES {
        let mut new_player_1 = player_1.clone();
        let mut new_player_2 = player_2.clone();

        let player = match turn {
            Turn::Player1 => &mut new_player_1,
            Turn::Player2 => &mut new_player_2,
        };

        let win = player.play(dice);

        if win {
            match player.id {
                1 => total_player_1_wins += times,
                2 => total_player_2_wins += times,
                _ => panic!(),
            }
        } else {
            let (player_1_wins, player_2_wins) = compute(new_player_1, new_player_2, next_turn);
            total_player_1_wins += player_1_wins * times;
            total_player_2_wins += player_2_wins * times;
        }
    }

    (total_player_1_wins, total_player_2_wins)
}

fn part1(player_1_position: usize, player_2_position: usize) -> usize
{
    let dices = &mut (1..=100).cycle().enumerate().map(|(index, dice)| (index + 1, dice));

    let mut player_1 = Player { id: 1, position: player_1_position, score: 0, turn: 0, win_condition: 1000 };
    let mut player_2 = Player { id: 2, position: player_2_position, score: 0, turn: 0, win_condition: 1000 };

    let (player_1, player_2, rolls) = loop {
        let (_, a) = dices.next().unwrap();
        let (_, b) = dices.next().unwrap();
        let (rolls, c) = dices.next().unwrap();

        let win = player_1.play(a + b + c);

        if win {
            break (player_1.score, player_2.score, rolls);
        }

        let (_, a) = dices.next().unwrap();
        let (_, b) = dices.next().unwrap();
        let (rolls, c) = dices.next().unwrap();

        let win = player_2.play(a + b + c);

        if win {
            break (player_1.score, player_2.score, rolls);
        }
    };
    
    min(player_1, player_2) * rolls
}

fn dices() -> Vec<usize>
{
    let mut dices = vec![];
    for a in [1, 2, 3] {
        for b in [1, 2, 3] {
            for c in [1, 2, 3] {
                dices.push(a + b + c);
            }
        }
    }

    dices
}

#[test]
fn it_works()
{
    assert_eq!(739785, part1(4, 8));
    assert_eq!(444356092776315_usize, part2(4, 8));
}