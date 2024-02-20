use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::bfs;

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
    is_dead: bool,
}

impl Battler {
    fn new(battler_type: BattlerType, position: IVec2, health: i32, attack: i32) -> Self {
        Self {
            battler_type,
            position,
            health,
            attack,
            is_dead: false,
        }
    }

    fn hit(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.is_dead = true;
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

    #[allow(dead_code)]
    fn width(&self) -> i32 {
        self.terrain.iter().map(|vec| vec.x).max().unwrap() + 1
    }

    #[allow(dead_code)]
    fn height(&self) -> i32 {
        self.terrain.iter().map(|vec| vec.y).max().unwrap() + 1
    }

    fn successors(&self, pos: &IVec2) -> Vec<IVec2> {
        let dirs = [
            IVec2::new(0, -1) + *pos,
            IVec2::new(-1, 0) + *pos,
            IVec2::new(1, 0) + *pos,
            IVec2::new(0, 1) + *pos,
        ];

        dirs.iter()
            .filter(|&d| {
                !self.terrain.contains(d)
                    && !self.battlers.iter().any(|b| b.position == *d && !b.is_dead)
            })
            .cloned()
            .collect()
    }

    fn choose_target(&self, battler: usize) -> Option<Vec<IVec2>> {
        let dirs = [
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, 1),
        ];

        let battler_obj = self.battlers[battler];

        let open_adjacents = self
            .battlers
            .iter()
            .filter(|b| b.battler_type != battler_obj.battler_type && !b.is_dead)
            .map(|b| {
                dirs.iter()
                    .filter_map(|d| {
                        let adjacent = *d + b.position;
                        if !self.terrain.contains(&adjacent)
                            && !self
                                .battlers
                                .iter()
                                .any(|b| b.position == adjacent && !b.is_dead)
                        {
                            Some(adjacent)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut shortest_paths = Vec::new();

        //for other in open_adjacents.iter().flatten() {
        //    let shortest_path: Option<Vec<IVec2>> = bfs(
        //        &battler_obj.position,
        //        |pos| self.successors(pos),
        //        |pos| pos == other,
        //    );
        //
        //    if shortest_path.is_some() {
        //        shortest_paths.push(shortest_path.unwrap());
        //    }
        //}

        let battler_open_adjacents = dirs
            .iter()
            .filter_map(|d| {
                let adjacent = *d + battler_obj.position;
                if !self.terrain.contains(&adjacent)
                    && !self
                        .battlers
                        .iter()
                        .any(|b| b.position == adjacent && !b.is_dead)
                {
                    Some(adjacent)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for other in open_adjacents.iter().flatten().unique() {
            for &battler_open_adjacent in battler_open_adjacents.iter() {
                let shortest_path: Option<Vec<IVec2>> = bfs(
                    &battler_open_adjacent,
                    |pos| self.successors(pos),
                    |pos| pos == other,
                );

                if let Some(path) = shortest_path {
                    shortest_paths.push(path);
                }
            }
        }

        shortest_paths.sort_by(|a, b| {
            let a_last = a.last().unwrap();
            let b_last = b.last().unwrap();
            a.len()
                .cmp(&b.len())
                .then(a_last.y.cmp(&b_last.y).then(a_last.x.cmp(&b_last.x)))
                .then(a[0].y.cmp(&b[0].y).then(a[0].x.cmp(&b[0].x)))
        });

        //if shortest_paths.len() > 0 {
        //    if shortest_paths
        //        .iter()
        //        .filter(|p| p.len() == shortest_paths[0].len())
        //        .count()
        //        > 1
        //    {
        //        for path in shortest_paths
        //            .iter()
        //            .filter(|p| p.len() == shortest_paths[0].len())
        //        {
        //            println!("{:?}", &path);
        //        }
        //
        //        println!();
        //    }
        //}

        /*
        if self.rounds == 15 {
            dbg!(&battler_obj);
            dbg!(&self.battlers);
            dbg!(&open_adjacents);
            dbg!(&shortest_paths);
        }

        if self.rounds == 25 {
            todo!();
        }
        */
        if !shortest_paths.is_empty() {
            Some(shortest_paths[0].clone())
        } else {
            None
        }
    }

    fn attackable(&mut self, battler: &Battler) -> Option<&mut Battler> {
        let dirs = [
            IVec2::new(0, -1) + battler.position,
            IVec2::new(-1, 0) + battler.position,
            IVec2::new(1, 0) + battler.position,
            IVec2::new(0, 1) + battler.position,
        ];

        self.battlers
            .iter_mut()
            .filter(|b| {
                dirs.contains(&b.position) && b.battler_type != battler.battler_type && !b.is_dead
            })
            .sorted_by(|a, b| {
                a.health
                    .cmp(&b.health)
                    .then(a.position.y.cmp(&b.position.y))
                    .then(a.position.x.cmp(&b.position.x))
            })
            .next()
    }

    fn take_turn(&mut self, battler: usize) {
        let battler_obj = self.battlers[battler];

        if battler_obj.is_dead {
            return;
        }

        if let Some(attackable) = self.attackable(&battler_obj) {
            attackable.hit(battler_obj.attack);
            return;
        }

        if let Some(target_path) = self.choose_target(battler) {
            self.battlers[battler].position = target_path[0];
        }

        let battler_obj = self.battlers[battler];

        if let Some(attackable) = self.attackable(&battler_obj) {
            attackable.hit(battler_obj.attack);
        }
    }

    fn victory_check(&mut self) -> bool {
        if self
            .battlers
            .iter()
            .filter(|b| !b.is_dead)
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
        self.rounds += 1;

        self.battlers.sort_by(|&a, &b| {
            a.position
                .y
                .cmp(&b.position.y)
                .then(a.position.x.cmp(&b.position.x))
        });

        for i in 0..self.battlers.len() {
            if self.victory_check() {
                return;
            }

            self.take_turn(i);
        }
    }

    #[allow(dead_code)]
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
                let battler = self
                    .battlers
                    .iter()
                    .find(|g| g.position == tile && !g.is_dead);

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

fn parse_battle(
    input: &str,
    starting_health: i32,
    goblin_starting_attack: i32,
    elf_starting_attack: i32,
) -> Option<Battle> {
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
                    goblin_starting_attack,
                )),
                'E' => battle.battlers.push(Battler::new(
                    BattlerType::Elf,
                    IVec2::new(x as i32, y as i32),
                    starting_health,
                    elf_starting_attack,
                )),
                '.' => {}
                _ => panic!("Invalid tile"),
            }
        }
    }

    Some(battle)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut battle = parse_battle(input, 200, 3, 3)?;

    //battle.draw();

    while !battle.battle_over {
        battle.round();
        //battle.draw();
    }

    let winning_team_health: i32 = battle
        .battlers
        .iter()
        .filter(|b| !b.is_dead)
        .map(|b| b.health)
        .sum();

    Some((battle.rounds - 1) * winning_team_health as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    for attack_power in 4.. {
        let mut battle = parse_battle(input, 200, 3, attack_power)?;
        let elves_count_before = battle
            .battlers
            .iter()
            .filter(|b| b.battler_type == BattlerType::Elf && !b.is_dead)
            .count();

        //battle.draw();

        while !battle.battle_over {
            battle.round();
            //battle.draw();
        }

        let elves_count_after = battle
            .battlers
            .iter()
            .filter(|b| b.battler_type == BattlerType::Elf && !b.is_dead)
            .count();

        let winning_team_health: i32 = battle
            .battlers
            .iter()
            .filter(|b| !b.is_dead)
            .map(|b| b.health)
            .sum();

        if elves_count_before == elves_count_after {
            return Some((battle.rounds - 1) * winning_team_health as usize);
        }
    }

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
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36334));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(27755));
    }

    #[test]
    fn test_part_one_4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(28944));
    }

    #[test]
    fn test_part_one_5() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(18740));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4988));
    }
}
