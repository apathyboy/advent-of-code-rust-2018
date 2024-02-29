use std::cmp::Reverse;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, Clone)]
enum GroupType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    id: usize,
    group_type: GroupType,
    units: usize,
    hit_points: usize,
    damage: usize,
    damage_type: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }
}

fn parse_group(input: &str) -> Option<Group> {
    let (units, rest) = input.split_once(" units each with ")?;
    let (hit_points, mut rest) = rest.split_once(" hit points ")?;
    let weaknesses_immunities_str = rest.split_once(" with an attack that does ");

    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();

    if weaknesses_immunities_str.is_some() {
        let (weaknesses_immunities_str, more) = weaknesses_immunities_str.unwrap();
        rest = more;

        let weaknesses_immunities_str = weaknesses_immunities_str
            .trim()
            .strip_prefix("(")?
            .strip_suffix(")")?;

        for mut attr in weaknesses_immunities_str.split("; ") {
            if attr.starts_with("weak to ") {
                attr = attr.strip_prefix("weak to ")?;
                weaknesses = attr.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>();
            }

            if attr.starts_with("immune to ") {
                attr = attr.strip_prefix("immune to ")?;
                immunities = attr.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>();
            }
        }
    } else {
        rest = rest.strip_prefix("with an attack that does ")?;
    }

    let (damage, rest) = rest.split_once(" ")?;
    let (damage_type, rest) = rest.split_once(" damage at initiative ")?;
    let initiative = rest.parse().ok()?;

    Some(Group {
        id: 0,
        group_type: GroupType::ImmuneSystem,
        units: units.parse().ok()?,
        hit_points: hit_points.parse().ok()?,
        damage: damage.parse().ok()?,
        damage_type: damage_type.to_string(),
        initiative,
        weaknesses,
        immunities,
    })
}

fn parse_input(input: &str) -> Option<Vec<Group>> {
    let (immune_system, infection) = input.split_once("\n\n")?;

    let mut immune_system = immune_system
        .lines()
        .skip(1)
        .enumerate()
        .filter_map(|(i, line)| parse_group(line).map(|g| Group { id: i + 1, ..g }))
        .collect::<Vec<_>>();

    let infection = infection
        .lines()
        .skip(1)
        .enumerate()
        .filter_map(|(i, line)| {
            parse_group(line).map(|g| Group {
                id: i + 1,
                group_type: GroupType::Infection,
                ..g
            })
        })
        .collect::<Vec<_>>();

    immune_system.extend(infection);

    Some(immune_system)
}

fn battle(mut units: Vec<Group>) -> (Option<GroupType>, usize) {
    loop {
        units.sort_by_key(|g| Reverse((g.effective_power(), g.initiative)));

        let mut targets = vec![None; units.len()];

        for (j, u) in units.iter().enumerate() {
            let mut best_damage = 0;
            for (i, v) in units.iter().enumerate() {
                if u.group_type == v.group_type || targets.contains(&Some(i)) || v.units == 0 {
                    continue;
                }

                let damage = if v.weaknesses.contains(&u.damage_type) {
                    u.effective_power() * 2
                } else if v.immunities.contains(&u.damage_type) {
                    0
                } else {
                    u.effective_power()
                };

                if damage > best_damage {
                    best_damage = damage;
                    targets[j] = Some(i);
                }
            }
        }

        let mut attackers = (0..units.len()).collect::<Vec<_>>();
        attackers.sort_by_key(|&i| Reverse(units[i].initiative));

        let mut any_die = false;

        for atk_idx in attackers {
            if units[atk_idx].units == 0 {
                continue;
            }

            if let Some(j) = targets[atk_idx] {
                let damage = if units[j].weaknesses.contains(&units[atk_idx].damage_type) {
                    units[atk_idx].effective_power() * 2
                } else if units[j].immunities.contains(&units[atk_idx].damage_type) {
                    0
                } else {
                    units[atk_idx].effective_power()
                };

                let mut def = units[j].clone();

                def.units = def.units.saturating_sub(damage / def.hit_points);
                any_die = any_die || damage > def.hit_points;

                units[j] = def;
            }
        }

        if !any_die {
            return (None, 0);
        }

        let immune_system_units = units
            .iter()
            .filter(|g| g.group_type == GroupType::ImmuneSystem && g.units > 0)
            .map(|g| g.units)
            .sum::<usize>();

        let infection_units = units
            .iter()
            .filter(|g| g.group_type == GroupType::Infection && g.units > 0)
            .map(|g| g.units)
            .sum::<usize>();

        if immune_system_units == 0 && infection_units == 0 {
            return (None, 0);
        } else if immune_system_units == 0 {
            return (Some(GroupType::Infection), infection_units);
        } else if infection_units == 0 {
            return (Some(GroupType::ImmuneSystem), immune_system_units);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let units = parse_input(input)?;

    Some(battle(units).1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let units = parse_input(input)?;
    Some(
        (1..)
            .find_map(|b| {
                let mut units = units.clone();
                units
                    .iter_mut()
                    .filter(|u| u.group_type == GroupType::ImmuneSystem)
                    .for_each(|u| u.damage += b);
                match battle(units) {
                    (Some(GroupType::ImmuneSystem), rem) => Some(rem),
                    _ => None,
                }
            })
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5216));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
