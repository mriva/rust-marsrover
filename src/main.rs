struct Planet {
    w: u8,
    h: u8
}

struct Position {
    x: u8,
    y: u8
}

enum Direction {
    N,
    E,
    S,
    W
}

enum Command {
    F,
    B,
    L,
    R
}

struct Rover {
    planet: Planet,
    position: Position,
    direction: Direction
}

fn main() {
    let rover = create_rover();
}

fn create_rover() -> Rover {
    let planet = Planet {w: 4, h: 5};
    let starting_position = Position {x: 0, y: 0};
    let direction = Direction::N;

    let rover = Rover {
        planet,
        position: starting_position,
        direction,
    };

    rover
}

fn execute(rover: Rover, command: Command) -> Rover {
    match command {
        Command::F => forward(rover),
        Command::B => backward(rover),
        Command::L => rotate_left(rover),
        Command::R => rotate_right(rover),
    }
}

fn forward(rover: Rover) -> Rover {
    let new_position = match rover.direction {
        Direction::N => Position { y: rover.position.y + 1, ..rover.position },
        Direction::E => Position { x: rover.position.x + 1, ..rover.position },
        Direction::S => Position { y: rover.position.y - 1, ..rover.position },
        Direction::W => Position { x: rover.position.x - 1, ..rover.position },
    };

    Rover {
        position: new_position,
        ..rover
    }
}

fn backward(rover: Rover) -> Rover {
    rover
}

fn rotate_left(rover: Rover) -> Rover {
    let new_direction = match rover.direction {
        Direction::N => Direction::W,
        Direction::E => Direction::N,
        Direction::S => Direction::E,
        Direction::W => Direction::S,
    };

    Rover {
        direction: new_direction,
        ..rover
    }
}

fn rotate_right(rover: Rover) -> Rover {
    let new_direction = match rover.direction {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    };

    Rover {
        direction: new_direction,
        ..rover
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_rover() {
        assert_eq!(0, create_rover().position.x);
    }

    #[test]
    fn rotate_right_command() {
        /*
           Planet: 5 4
           Rover: 0 0 N
           Command: R
           --
           Rover: 0 0 E
           */
        let planet = Planet {w: 4, h: 5};
        let starting_position = Position {x: 0, y: 0};
        let direction = Direction::N;

        let rover = Rover {
            planet,
            position: starting_position,
            direction,
        };

        assert_eq!(0, rover.position.x);
        assert_eq!(0, rover.position.y);
        assert!(matches!(execute(rover, Command::R).direction, Direction::E));
    }

    #[test]
    fn rotate_left_command() {
        /*
            Planet: 5 4
            Rover: 0 0 N
            Command: L
            --
            Rover: 0 0 W
        */
        let planet = Planet {w: 4, h: 5};
        let starting_position = Position {x: 0, y: 0};
        let direction = Direction::N;

        let rover = Rover {
            planet,
            position: starting_position,
            direction,
        };

        assert_eq!(0, rover.position.x);
        assert_eq!(0, rover.position.y);
        assert!(matches!(execute(rover, Command::L).direction, Direction::W));
    }
}
