use std::convert::{From, TryFrom};

//--------------------------------------------------------------------------------------
//-- Direction Enum
//-- Indicates falling to the left, the right, or standing still
//--------------------------------------------------------------------------------------
enum Direction {
    Left,
    Right,
    Still,
}

/// Conversion from character to a Direction, with error-checking
impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            '.' => Ok(Direction::Still),
            _ => Err("Malformed input string."),
        }
    }
}


//--------------------------------------------------------------------------------------
//-- Domino Struct
//-- Stores the original 'state' of the domino as `starting_direction`, the force on
//-- the domino from the right, and the force on the domino from the left. The two
//-- forces can be used to calculate the final state of the domino.
//--------------------------------------------------------------------------------------
#[allow(dead_code)]
struct Domino {
    starting_direction: Direction,
    force_from_right: u8,
    force_from_left: u8,
}

/// Conversion from char to Domino, with error-checking
impl TryFrom<char> for Domino {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let starting_direction = Direction::try_from(c)?;
        let (force_from_right, force_from_left) = match starting_direction {
            Direction::Left  => (0,       u8::MAX),
            Direction::Right => (u8::MAX, 0),
            Direction::Still => (0,       0)
        };
        Ok(Domino { starting_direction, force_from_left, force_from_right })
    }
}

/// The 'defaul' Domino is a domino standing still
impl Default for Domino {
    fn default() -> Self {
        Domino { 
            starting_direction: Direction::Still,
            force_from_right: 0,
            force_from_left: 0,
        }
    }
}

/// Convert a reference to a Domino to a char, no error-checking needed
impl From<&Domino> for char {
    fn from(domino: &Domino) -> char {
        if domino.force_from_right > domino.force_from_left { return 'R'; }
        if domino.force_from_right < domino.force_from_left { return 'L'; }
        '.'
    }
}


//--------------------------------------------------------------------------------------
#[allow(dead_code)]
fn topple(input: &str) -> String {
    // Convert the input string slice into a Vec of Dominos. Any malformed characters 
    // that return an Err are replaced with the 'default' still Domino.
    let mut dominoes: Vec<_> = input.chars()
        .map(Domino::try_from)
        .map(|x| x.unwrap_or_default())
        .collect();

    // Calculate the force applied from the right for each Domino, moving from 
    // left to right
    let mut current_force = 0;
    for domino in dominoes.iter_mut() {
        current_force = match domino.starting_direction {
            Direction::Right => u8::MAX,
            Direction::Left  => 0,
            Direction::Still => current_force.saturating_sub(1),
        };
        domino.force_from_right = current_force;
    }

    // Calculate the force applied from the left for each Domino, moving from
    // right to left
    current_force = 0;
    for domino in dominoes.iter_mut().rev() {
        current_force = match domino.starting_direction {
            Direction::Right => 0,
            Direction::Left  => u8::MAX,
            Direction::Still => current_force.saturating_sub(1),
        };
        domino.force_from_left = current_force;
    }

    // Convert the Vec<Domino> to a String for return
    dominoes.iter().map(char::from).collect()
}


//--------------------------------------------------------------------------------------
//-- Tests
//--------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        // The test cases given in the challenge
        assert_eq!(topple(".L.R....L"), "LL.RRRLLL".to_string());
        assert_eq!(topple("L.......R"), "L.......R".to_string());
        assert_eq!(topple("R........"), "RRRRRRRRR".to_string());
        assert_eq!(topple("........L"), "LLLLLLLLL".to_string());
        assert_eq!(topple("RLRLRLRLR"), "RLRLRLRLR".to_string());
        assert_eq!(topple("R..L.R..L"), "RRLL.RRLL".to_string());
        assert_eq!(topple("...L.R..."), "LLLL.RRRR".to_string());
        assert_eq!(topple("R.......L"), "RRRR.LLLL".to_string());
    }

    #[test]
    fn test_malformed_entry() {
        // An extra test case for error-checking on malformed input
        assert_eq!(topple("R..G*P..L"), "RRRR.LLLL".to_string());
    }
}

