#![allow(unused)]
struct Planet {
    w: u8,
    h: u8
}

impl Planet {
    fn next_north(&self, position: Position) -> Position {
        if position.y + 1 == self.h {
            Position { y: 0, ..position }
        } else {
            Position { y: position.y + 1, ..position }
        }
    }
    fn next_east(&self, position: Position) -> Position {
        if position.x + 1 == self.w {
            Position { x: 0, ..position }
        } else {
            Position { x: position.x + 1, ..position }
        }
    }
    fn next_south(&self, position: Position) -> Position {
        if position.y == 0 {
            Position { y: self.h - 1, ..position }
        } else {
            Position { y: position.y - 1, ..position }
        }
    }
    fn next_west(&self, position: Position) -> Position {
        if position.x == 0 {
            Position { x: self.w - 1, ..position }
        } else {
            Position { x: position.x - 1, ..position }
        }
    }
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
    position: Position,
    direction: Direction
}

fn execute(rover: Rover, planet: Planet, command: Command) -> Rover {
    match command {
        Command::F => forward(rover, planet),
        Command::B => backward(rover, planet),
        Command::L => rotate_left(rover),
        Command::R => rotate_right(rover),
    }
}

fn forward(rover: Rover, planet: Planet) -> Rover {
    let new_position = match rover.direction {
        Direction::N => planet.next_north(rover.position),
        Direction::E => planet.next_east(rover.position),
        Direction::S => planet.next_south(rover.position),
        Direction::W => planet.next_west(rover.position),
    };

    Rover {
        position: new_position,
        ..rover
    }
}

fn backward(rover: Rover, planet: Planet) -> Rover {
    let new_position = match rover.direction {
        Direction::N => planet.next_south(rover.position),
        Direction::E => planet.next_west(rover.position),
        Direction::S => planet.next_north(rover.position),
        Direction::W => planet.next_east(rover.position),
    };

    Rover {
        position: new_position,
        ..rover
    }
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

fn main() {
    println!("To boldly go where no man has gone before.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_command() {
        /*
           Planet: 5 4
           Rover: 0 0 N
           Command: R
           --
           Rover: 0 0 E
           */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::N,
        };

        let new_rover = execute(rover, planet, Command::R);
        let expected = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::E,
        };
        assert!(matches!(expected, new_rover));
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
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::N,
        };

        let new_rover = execute(rover, planet, Command::L);
        let expected = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::W,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn move_forward_command() {
        /*
            Planet: 5 4
            Rover: 0 1 N
            Command: F
            --
            Rover: 0 2 N
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 1 },
            direction: Direction::N,
        };

        let new_rover = execute(rover, planet, Command::F);
        let expected = Rover {
            position: Position { x: 0, y: 2 },
            direction: Direction::N,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn move_forward_southbound_ommand() {
        /*
            Planet: 5 4
            Rover: 0 1 S
            Command: F
            --
            Rover: 0 0 S
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 1 },
            direction: Direction::S,
        };

        let new_rover = execute(rover, planet, Command::F);
        let expected = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::S,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn move_backward_command() {
        /*
            Planet: 5 4
            Rover: 0 1 N
            Command: B
            --
            Rover: 0 0 N
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 1 },
            direction: Direction::N,
        };

        let new_rover = execute(rover, planet, Command::B);
        let expected = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::N,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn wrap_on_north() {
        /*
            Planet: 5 4
            Rover: 0 3 N
            Command: F
            --
            Rover: 0 0 N
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 3 },
            direction: Direction::N,
        };

        let new_rover = execute(rover, planet, Command::F);
        let expected = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::N,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn wrap_on_south() {
        /*
            Planet: 5 4
            Rover: 0 0 S
            Command: F
            --
            Rover: 0 3 S
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::S,
        };

        let new_rover = execute(rover, planet, Command::L);
        let expected = Rover {
            position: Position { x: 0, y: 3 },
            direction: Direction::S,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn wrap_on_east() {
        /*
            Planet: 5 4
            Rover: 4 1 E
            Command: F
            --
            Rover: 0 1 E
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 4, y: 1 },
            direction: Direction::E,
        };

        let new_rover = execute(rover, planet, Command::L);
        let expected = Rover {
            position: Position { x: 0, y: 1 },
            direction: Direction::E,
        };
        assert!(matches!(expected, new_rover));
    }

    #[test]
    fn wrap_on_west() {
        /*
            Planet: 5 4
            Rover: 0 1 W
            Command: F
            --
            Rover: 4 1 W
        */
        let planet = Planet { w: 5, h: 4 };
        let rover = Rover {
            position: Position { x: 0, y: 1 },
            direction: Direction::W,
        };

        let new_rover = execute(rover, planet, Command::L);
        let expected = Rover {
            position: Position { x: 4, y: 1 },
            direction: Direction::W,
        };
        assert!(matches!(expected, new_rover));
    }
}
