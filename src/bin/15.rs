use glam::IVec2;
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BattlerType {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Copy)]
struct Battler {
    battler_type: BattlerType,
    position: IVec2,
    health: i32,
    attack: i32,
}

impl Battler {
    fn new(battler_type: BattlerType, position: IVec2, health: i32, attack: i32) -> Self {
        Self {
            battler_type,
            position,
            health,
            attack,
        }
    }
}

#[derive(Debug, Clone)]
struct Battle {
    terrain: Vec<IVec2>,
    battlers: Vec<Battler>,
    rounds: usize,
    battle_over: bool,
}

impl Battle {
    fn new() -> Self {
        Self {
            terrain: Vec::new(),
            battlers: Vec::new(),
            rounds: 0,
            battle_over: false,
        }
    }

    fn width(&self) -> i32 {
        self.terrain.iter().map(|vec| vec.x).max().unwrap() + 1
    }

    fn height(&self) -> i32 {
        self.terrain.iter().map(|vec| vec.y).max().unwrap() + 1
    }

    fn take_turn(&mut self, battler: usize) {
        // move
        // attack

        todo!()
    }

    fn victory_check(&mut self) -> bool {
        if self
            .battlers
            .iter()
            .map(|b| b.battler_type)
            .unique()
            .count()
            == 1
        {
            self.battle_over = true;
        }

        self.battle_over
    }

    fn round(&mut self) {
        self.battlers.sort_by(|&a, &b| {
            a.position
                .y
                .cmp(&b.position.y)
                .then(a.position.x.cmp(&b.position.x))
        });

        for i in 0..self.battlers.len() {
            self.take_turn(i);

            if self.victory_check() {
                return;
            }
        }

        self.rounds += 1;
    }

    fn draw(&self) {
        let width = self.width();
        let height = self.height();

        if self.rounds == 0 {
            println!("Initially:");
        } else if self.rounds == 1 {
            println!("After 1 round:");
        } else {
            println!("After {} rounds:", self.rounds);
        }

        for y in 0..height {
            let mut row_battlers = Vec::new();

            for x in 0..width {
                let tile = IVec2::new(x, y);
                let battler = self.battlers.iter().find(|g| g.position == tile);

                if self.terrain.contains(&tile) {
                    print!("#");
                } else if let Some(battler) = battler {
                    let battler_type = if battler.battler_type == BattlerType::Goblin {
                        "G"
                    } else {
                        "E"
                    };

                    print!("{battler_type}");
                    row_battlers.push(format!("{}({})", battler_type, battler.health));
                } else {
                    print!(".");
                }
            }

            println!("   {}", row_battlers.iter().join(", "));
        }

        println!();
    }
}

fn parse_battle(input: &str, starting_health: i32, starting_attack: i32) -> Option<Battle> {
    let mut battle = Battle::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    battle.terrain.push(IVec2::new(x as i32, y as i32));
                }
                'G' => battle.battlers.push(Battler::new(
                    BattlerType::Goblin,
                    IVec2::new(x as i32, y as i32),
                    starting_health,
                    starting_attack,
                )),
                'E' => battle.battlers.push(Battler::new(
                    BattlerType::Elf,
                    IVec2::new(x as i32, y as i32),
                    starting_health,
                    starting_attack,
                )),
                '.' => {}
                _ => panic!("Invalid tile"),
            }
        }
    }

    Some(battle)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut battle = parse_battle(input, 200, 3)?;

    battle.draw();

    while !battle.battle_over {
        battle.round();
        battle.draw();
    }

    let winning_team_health: i32 = battle.battlers.iter().map(|b| b.health).sum();

    Some(battle.rounds * winning_team_health as usize)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(27730));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
