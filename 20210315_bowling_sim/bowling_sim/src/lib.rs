//! Write a class named “BOWLING_GAME” that has two methods:
//! - ROLL(PINS : integer)
//!     - This is called each time the player rolls a ball.
//!     - The argument is the number of pins knocked down.
//! - SCORE( ) : integer
//!     - This is called only at the very end of the game.
//!     - It returns the total score for that game.

pub mod err;
pub mod frame;

use crate::err::BowlingError;
use crate::frame::{Frame, FrameStatus};

type Result<T> = std::result::Result<T, BowlingError>;

/// Trait to provide functions related to a bowling Game
pub trait Game {

    /// Roll the "ball", knock down pins. This is the main function for advancing
    /// the game.
    fn roll(&mut self, pins: usize) -> Result<()>;

    /// Indicates whether the game is over, i.e. all ten frames have been played.
    fn finished(&self) -> bool;

    /// Indicates whether the current frame is still open, i.e. can more balls
    /// be rolled on the current frame
    fn current_frame_open(&self) -> bool;

    /// Given the current state of the game, returns the next frame to bowl on.
    /// Since the final frame is treated differently, this is either a regular
    /// frame or a final frame.
    fn next_frame(&mut self) -> Result<Frame>;

    /// Calculates and returns the current score of the game.
    fn score(&self) -> usize;
}

impl Game for Vec<Frame> {
    fn roll(&mut self, pins: usize) -> Result<()> {
        let mut current_frame = if self.current_frame_open() {
            self.pop().unwrap()
        } else {
            self.next_frame()?
        };

        current_frame.bowl(pins)?;
        let previous1_frame = self.pop();
        let previous2_frame = self.pop();

        if let Some(mut frame) = previous2_frame {
            frame.add_bonus(pins).ok();
            self.push(frame);
        }
        if let Some(mut frame) = previous1_frame {
            frame.add_bonus(pins).ok();
            self.push(frame);
        }

        self.push(current_frame);
        Ok(())
    }

    fn finished(&self) -> bool {
        let frame_count = self.len();
        frame_count == 10 && !self.current_frame_open()
    }

    fn current_frame_open(&self) -> bool {
        match self.last() {
            None => false,
            Some(frame_ref) => matches!(
                frame_ref.status(),
                FrameStatus::Open(_) | FrameStatus::Final(_)
            ),
        }
    }

    fn next_frame(&mut self) -> Result<Frame> {
        match self.len() {
            0..=8 => Ok(Frame::default()),
            9 => Ok(Frame::final_frame()),
            _ => Err(BowlingError::new("Cannot add more frames to this game.")),
        }
    }

    fn score(&self) -> usize {
        self.iter().fold(0, |t, n| t + n.score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gutter_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        for _ in 0..20 {
            new_game.roll(0).unwrap();
        }
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 0);
    }

    #[test]
    fn test_single_pin_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        new_game.roll(1).unwrap();
        for _ in 1..20 {
            new_game.roll(0).unwrap();
        }
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 1);
    }

    #[test]
    fn test_single_spare_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        new_game.roll(4).unwrap();
        new_game.roll(6).unwrap();
        new_game.roll(2).unwrap();
        for _ in 3..20 {
            new_game.roll(0).unwrap();
        }
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 14);
    }

    #[test]
    fn test_single_strike_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        new_game.roll(10).unwrap();
        new_game.roll(6).unwrap();
        new_game.roll(2).unwrap();
        for _ in 4..20 {
            new_game.roll(0).unwrap();
        }
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 26);
    }

    #[test]
    fn test_perfect_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        for _ in 0..12 {
            new_game.roll(10).unwrap();
        }
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 300);
    }

    #[test]
    fn test_sanity_game() {
        let mut new_game: Vec<Frame> = Vec::new();
        new_game.roll(1).unwrap();
        new_game.roll(4).unwrap();
        new_game.roll(4).unwrap();
        new_game.roll(5).unwrap();
        new_game.roll(6).unwrap();
        new_game.roll(4).unwrap();
        new_game.roll(5).unwrap();
        new_game.roll(5).unwrap();
        new_game.roll(10).unwrap();
        new_game.roll(0).unwrap();
        new_game.roll(1).unwrap();
        new_game.roll(7).unwrap();
        new_game.roll(3).unwrap();
        new_game.roll(6).unwrap();
        new_game.roll(4).unwrap();
        new_game.roll(10).unwrap();
        new_game.roll(2).unwrap();
        new_game.roll(8).unwrap();
        new_game.roll(6).unwrap();
        assert_eq!(new_game.finished(), true);
        assert_eq!(new_game.score(), 133);
    }
}
