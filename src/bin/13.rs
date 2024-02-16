use glam::IVec2;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Cart {
    position: IVec2,
    direction: IVec2,
    next_turn_decision: u8,
    is_crashed: bool,
}

impl Cart {
    fn new(position: IVec2, direction: IVec2) -> Self {
        Self {
            position,
            direction,
            next_turn_decision: 0,
            is_crashed: false,
        }
    }
}

struct TrackPiece {
    position: IVec2,
    neighbors: Vec<IVec2>,
    piece_type: char,
}

impl TrackPiece {
    fn new(x: i32, y: i32, piece_type: char) -> Self {
        Self {
            position: IVec2::new(x, y),
            neighbors: Vec::new(),
            piece_type,
        }
    }
}

struct Track {
    pieces: Vec<TrackPiece>,
}

impl Track {
    fn new() -> Self {
        Self { pieces: Vec::new() }
    }
}

fn parse(input: &str) -> Option<(Track, Vec<Cart>)> {
    let mut carts = Vec::new();
    let mut track = Track::new();

    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    for (y, line) in input.lines().enumerate() {
        let mut prev = ' ';

        for (x, c) in line.chars().enumerate() {
            if c == ' ' {
                prev = c;
                continue;
            }

            let mut piece = TrackPiece::new(x as i32, y as i32, c);

            match c {
                '/' => {
                    if prev == '-' {
                        piece.neighbors.push(piece.position + dirs[2]);
                        piece.neighbors.push(piece.position + dirs[0]);
                    } else {
                        piece.neighbors.push(piece.position + dirs[3]);
                        piece.neighbors.push(piece.position + dirs[1]);
                    }
                }
                '\\' => {
                    if prev == '-' {
                        piece.neighbors.push(piece.position + dirs[2]);
                        piece.neighbors.push(piece.position + dirs[1]);
                    } else {
                        piece.neighbors.push(piece.position + dirs[3]);
                        piece.neighbors.push(piece.position + dirs[0]);
                    }
                }
                '-' => {
                    piece.neighbors.push(piece.position + dirs[2]);
                    piece.neighbors.push(piece.position + dirs[3]);
                }
                '|' => {
                    piece.neighbors.push(piece.position + dirs[0]);
                    piece.neighbors.push(piece.position + dirs[1]);
                }
                '+' => {
                    piece.neighbors.push(piece.position + dirs[0]);
                    piece.neighbors.push(piece.position + dirs[1]);
                    piece.neighbors.push(piece.position + dirs[2]);
                    piece.neighbors.push(piece.position + dirs[3]);
                }
                '>' => {
                    piece.neighbors.push(piece.position + dirs[2]);
                    piece.neighbors.push(piece.position + dirs[3]);
                    piece.piece_type = '-';

                    carts.push(Cart::new(piece.position, dirs[3]));
                }
                '<' => {
                    piece.neighbors.push(piece.position + dirs[2]);
                    piece.neighbors.push(piece.position + dirs[3]);
                    piece.piece_type = '-';

                    carts.push(Cart::new(piece.position, dirs[2]));
                }
                '^' => {
                    piece.neighbors.push(piece.position + dirs[0]);
                    piece.neighbors.push(piece.position + dirs[1]);
                    piece.piece_type = '|';

                    carts.push(Cart::new(piece.position, dirs[0]));
                }
                'v' => {
                    piece.neighbors.push(piece.position + dirs[0]);
                    piece.neighbors.push(piece.position + dirs[1]);
                    piece.piece_type = '|';

                    carts.push(Cart::new(piece.position, dirs[1]));
                }
                _ => {}
            }

            track.pieces.push(piece);

            prev = c;
        }
    }

    Some((track, carts))
}

fn detect_crash(carts: &[Cart]) -> Option<IVec2> {
    for (i, cart) in carts.iter().enumerate() {
        for (j, other) in carts.iter().enumerate() {
            if i == j {
                continue;
            }

            if cart.position == other.position && !cart.is_crashed && !other.is_crashed {
                return Some(cart.position);
            }
        }
    }

    None
}

fn turn_left(direction: IVec2) -> Option<IVec2> {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    if direction == dirs[0] {
        Some(dirs[2])
    } else if direction == dirs[1] {
        Some(dirs[3])
    } else if direction == dirs[2] {
        Some(dirs[1])
    } else if direction == dirs[3] {
        Some(dirs[0])
    } else {
        panic!("invalid move");
    }
}

fn turn_right(direction: IVec2) -> Option<IVec2> {
    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    if direction == dirs[0] {
        Some(dirs[3])
    } else if direction == dirs[1] {
        Some(dirs[2])
    } else if direction == dirs[2] {
        Some(dirs[0])
    } else if direction == dirs[3] {
        Some(dirs[1])
    } else {
        panic!("invalid move");
    }
}

fn update_direction(track: &Track, cart: &Cart) -> Option<IVec2> {
    let track_piece = track.pieces.iter().find(|p| p.position == cart.position)?;

    let dirs = [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
    ];

    if track_piece.piece_type == '-'
        || track_piece.piece_type == '|'
        || track_piece.piece_type == '+'
    {
        Some(cart.direction)
    } else if track_piece.piece_type == '/' {
        if cart.direction == dirs[0] {
            Some(dirs[3])
        } else if cart.direction == dirs[1] {
            Some(dirs[2])
        } else if cart.direction == dirs[2] {
            Some(dirs[1])
        } else if cart.direction == dirs[3] {
            Some(dirs[0])
        } else {
            panic!("invalid move");
            //None
        }
    } else if track_piece.piece_type == '\\' {
        if cart.direction == dirs[0] {
            Some(dirs[2])
        } else if cart.direction == dirs[1] {
            Some(dirs[3])
        } else if cart.direction == dirs[2] {
            Some(dirs[0])
        } else if cart.direction == dirs[3] {
            Some(dirs[1])
        } else {
            panic!("invalid move");
            //None
        }
    } else {
        panic!("invalid move");
        //None
    }
}

fn move_cart(track: &Track, cart: &Cart) -> Option<Cart> {
    let track_piece = track.pieces.iter().find(|p| p.position == cart.position)?;

    let mut next_cart = *cart;

    if track_piece.neighbors.len() == 4 {
        // its an intersection
        match next_cart.next_turn_decision {
            0 => {
                next_cart.direction = turn_left(next_cart.direction)?;
                next_cart.position += next_cart.direction;
                next_cart.direction = update_direction(track, &next_cart)?;

                next_cart.next_turn_decision = 1;
            }
            1 => {
                next_cart.position += next_cart.direction;
                next_cart.direction = update_direction(track, &next_cart)?;

                next_cart.next_turn_decision = 2;
            }
            2 => {
                next_cart.direction = turn_right(next_cart.direction)?;
                next_cart.position += next_cart.direction;
                next_cart.direction = update_direction(track, &next_cart)?;

                next_cart.next_turn_decision = 0;
            }
            _ => panic!("Invalid turn choice"),
        }
    } else {
        // move forward
        next_cart.position += next_cart.direction;
        next_cart.direction = update_direction(track, &next_cart)?;
    }

    if next_cart.position == cart.position {
        panic!("invalid move occured");
    }

    Some(next_cart)
}

fn tick(track: &Track, carts: &mut [Cart]) -> Option<IVec2> {
    carts.sort_by(|a, b| {
        a.position
            .y
            .cmp(&b.position.y)
            .then(a.position.x.cmp(&b.position.x))
    });

    for i in 0..carts.len() {
        carts[i] = move_cart(track, &carts[i])?;

        let crashed = detect_crash(carts);

        if crashed.is_some() {
            return crashed;
        }
    }

    None
}

fn tick2(track: &Track, carts: &mut [Cart]) {
    carts.sort_by(|a, b| {
        a.position
            .y
            .cmp(&b.position.y)
            .then(a.position.x.cmp(&b.position.x))
    });

    for i in 0..carts.len() {
        carts[i] = move_cart(track, &carts[i]).unwrap();

        let crashed = detect_crash(carts);

        if let Some(crash) = crashed {
            for cart in carts.iter_mut() {
                if cart.position == crash {
                    cart.is_crashed = true;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (track, mut carts) = parse(input)?;

    loop {
        let result = tick(&track, &mut carts);

        if let Some(crash) = result {
            return Some(format!("{},{}", crash.x, crash.y));
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (track, mut carts) = parse(input)?;

    loop {
        tick2(&track, &mut carts);

        carts.retain(|c| !c.is_crashed);

        if carts.len() == 1 {
            return Some(format!("{},{}", carts[0].position.x, carts[0].position.y));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("7,3".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("6,4".to_owned()));
    }
}
