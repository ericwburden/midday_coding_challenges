//! Implements the [Frame](crate::frame::Frame) struct

use crate::err::BowlingError;

type Result<T> = std::result::Result<T, BowlingError>;

/// Enum represents the status of the frame:
/// - Open: Frame is ongoing (number of rolls remaining)
/// - Final: Frame is the final frame (number of rolls remaining)
/// - Bonus: Frame is still accepting bonus points (number of bonus applications remaining)
/// - Closed: No more points can be added to this frame
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameStatus {
    Open(usize),
    Final(usize),
    Bonus(usize),
    Closed,
}

/// Represents a single frame in a bowling game. Implements functions for updating
/// and modifying a frame. Contains the frames current status and number of pins
/// knocked down (score).
#[derive(Debug)]
pub struct Frame {
    status: FrameStatus,
    score: usize,
}

impl Default for Frame {
    fn default() -> Self {
        // This is always a valid FrameStatus, no error
        Self::new(FrameStatus::Open(2)).unwrap()
    }
}

impl Frame {
    /// Creates a new frame. `status` must be either FrameStatus::Open(2) or 
    /// FrameStatus::Final(3). Returns an error for any other status.
    pub fn new(status: FrameStatus) -> Result<Self> {
        match status {
            FrameStatus::Open(2) => Ok(Frame { status, score: 0 }),
            FrameStatus::Final(3) => Ok(Frame { status, score: 0 }),
            _ => Err(BowlingError::new(
                "Cannot create a new frame with that status.",
            )),
        }
    }

    /// Creates a new 'final' frame.
    pub fn final_frame() -> Self {
        Frame::new(FrameStatus::Final(3)).unwrap()
    }

    /// Attempt to add bonus points to a frame. Returns an error if attempting to
    /// add bonus points more than once (spare) or twice (strike).
    pub fn add_bonus(&mut self, pins: usize) -> Result<()> {
        if pins > 10 {
            return Err(BowlingError::new("Cannot add a bonus greater than 10."));
        }
        self.status = match self.status {
            FrameStatus::Bonus(b) => Frame::next_bonus_status(b - 1),
            _ => return Err(BowlingError::new("Cannot add bonus points to this frame.")),
        };
        self.score += pins; // Add bonus
        Ok(())
    }

    /// Attempts to bowl on a frame. Returns an error if attempting to bowl on 
    /// a frame more than twice (or thrice for the final frame). Also returns
    /// an error if attempting to knock down more pins than currently left in the
    /// frame.
    pub fn bowl(&mut self, pins: usize) -> Result<()> {
        let new_score = self.score + pins;
        let new_status = match self.status {
            FrameStatus::Open(r) => Frame::next_open_status(r - 1, new_score),
            FrameStatus::Final(r) => Frame::next_final_status(r - 1, new_score),
            _ => return Err(BowlingError::new("Cannot bowl on this frame.")),
        };

        if pins > self.pins_up()? {
            return Err(BowlingError::new(
                "You tried to knock down more pins than exist.",
            ));
        }

        self.score = new_score;
        self.status = new_status;
        Ok(())
    }

    /// Determins the next status for a frame with status = FrameStatus::Bonus()
    /// from the number of bonus rolls remaining
    fn next_bonus_status(bonus_remaining: usize) -> FrameStatus {
        if bonus_remaining == 0 {
            FrameStatus::Closed // Applied all possible bonus
        } else {
            FrameStatus::Bonus(bonus_remaining)
        }
    }

    /// Determines the next status for a frame with status = FrameStatus::Open()
    /// from the number of rolls remaining in the frame and the current score
    fn next_open_status(rolls_remaining: usize, score: usize) -> FrameStatus {
        if rolls_remaining == 1 && score == 10 {
            FrameStatus::Bonus(2) // You rolled a strike
        } else if rolls_remaining == 1 && score < 10 {
            FrameStatus::Open(rolls_remaining) // You have another chance
        } else if rolls_remaining == 0 && score == 10 {
            FrameStatus::Bonus(1) // You rolled a spare
        } else {
            FrameStatus::Closed // This frame is over
        }
    }

    /// Determines the next status for a frame with status = FrameStatus::Final()
    /// from the number of rolls remaining in the frame and the current score.
    fn next_final_status(rolls_remaining: usize, score: usize) -> FrameStatus {
        if rolls_remaining == 0 || (rolls_remaining == 1 && score < 10) {
            // If there are no more rolls remaining OR if you did not get at least
            // a spare on the first two rolls of the final frame...
            FrameStatus::Closed
        } else {
            FrameStatus::Final(rolls_remaining) // Keep rolling
        }
    }

    /// Identifies the number of pins available to be knocked down based on Frame
    /// state, as follows
    ///
    /// - If FrameState is Closed or Bonus, then no pins are available (this frame
    /// is over)
    /// - If FrameState is Open and there are two rolls remaining, then the frame
    /// is just starting and 10 pins are available
    /// - If FrameState is Open and there is one roll available, then the number
    /// of pins available is 10 - the pins knocked down on the first roll (current
    /// score)
    /// - If FrameState is Final and there are three rolls remaining, then the
    /// frame is just starting and 10 pins are available.
    /// - If FrameState is Final and there are two rolls remaining, then there
    /// are two possibilities. If the first roll was a strike (the score is 10),
    /// then all the pins are back up and 10 pins are available. If the first
    /// roll was not a strike (the score is not 10), then the number of pins
    /// available is 10 - the pins knocked down on the first roll.
    /// - If FrameState is Final and there is one roll remaining, then there
    /// are two possibilities. If the score is 10 (first two rolls made a spare)
    /// or 20 (two strikes), then all 10 pins are available. The other possibility
    /// is that the first roll was a strike and the second roll was not, in which
    /// case the number of pins available is 20 - 10 (for the strike) - the number
    /// of pins knocked down on the second roll.
    pub fn pins_up(&self) -> Result<usize> {
        let pins_remaining = match self.status {
            FrameStatus::Closed => 0,
            FrameStatus::Bonus(_) => 0,
            FrameStatus::Open(r) => match r {
                2 => 10,
                1 => 10 - self.score,
                _ => return Err(BowlingError::new("Frame is in an unsupported status.")),
            },
            FrameStatus::Final(r) => match r {
                3 => 10,
                2 => {
                    if self.score == 10 {
                        10
                    } else {
                        10 - self.score
                    }
                }
                1 => {
                    if [10, 20].contains(&self.score) {
                        10
                    } else {
                        20 - self.score
                    }
                }
                _ => return Err(BowlingError::new("Frame is in an unsupported status.")),
            },
        };
        Ok(pins_remaining)
    }

    /// Returns current status of this frame
    pub fn status(&self) -> FrameStatus {
        self.status
    }

    /// Returns current score of this frame
    pub fn score(&self) -> usize {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_frame_miss() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Open(1));
        assert_eq!(open_frame.score(), 5);

        open_frame.bowl(3).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Closed);
        assert_eq!(open_frame.score(), 8);
    }

    #[test]
    fn test_open_frame_spare() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Open(1));
        assert_eq!(open_frame.score(), 5);

        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(1));
        assert_eq!(open_frame.score(), 10);
    }

    #[test]
    fn test_open_frame_strike() {
        let mut open_frame = Frame::default();
        open_frame.bowl(10).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(2));
        assert_eq!(open_frame.score(), 10);
    }

    #[test]
    fn test_final_frame_miss() {
        let mut final_frame = Frame::final_frame();
        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Final(2));
        assert_eq!(final_frame.score(), 5);

        final_frame.bowl(4).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Closed);
        assert_eq!(final_frame.score(), 9);
    }

    #[test]
    fn test_final_frame_spare() {
        let mut final_frame = Frame::final_frame();
        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Final(2));
        assert_eq!(final_frame.score(), 5);

        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Final(1));
        assert_eq!(final_frame.score(), 10);

        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Closed);
        assert_eq!(final_frame.score(), 15);
    }

    #[test]
    fn test_final_frame_strike() {
        let mut final_frame = Frame::final_frame();
        final_frame.bowl(10).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Final(2));
        assert_eq!(final_frame.score(), 10);

        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Final(1));
        assert_eq!(final_frame.score(), 15);

        final_frame.bowl(5).unwrap();
        assert_eq!(final_frame.status(), FrameStatus::Closed);
        assert_eq!(final_frame.score(), 20);
    }

    #[test]
    fn test_bonus_frame_spare() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(1));
        assert_eq!(open_frame.score(), 10);

        open_frame.add_bonus(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Closed);
        assert_eq!(open_frame.score(), 15);
    }

    #[test]
    fn test_bonus_frame_strike() {
        let mut open_frame = Frame::default();
        open_frame.bowl(10).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(2));
        assert_eq!(open_frame.score(), 10);

        open_frame.add_bonus(10).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(1));
        assert_eq!(open_frame.score(), 20);

        open_frame.add_bonus(10).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Closed);
        assert_eq!(open_frame.score(), 30);
    }

    #[test]
    fn test_one_roll_overload() {
        let mut open_frame = Frame::default();
        assert_eq!(
            open_frame.bowl(11).err(),
            Some(BowlingError::new(
                "You tried to knock down more pins than exist."
            ))
        );
    }

    #[test]
    fn test_two_roll_overload() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        assert_eq!(
            open_frame.bowl(6).err(),
            Some(BowlingError::new(
                "You tried to knock down more pins than exist."
            ))
        );
    }

    #[test]
    fn test_bonus_overflow() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(1));
        assert_eq!(open_frame.score(), 10);

        assert_eq!(
            open_frame.add_bonus(11).err(),
            Some(BowlingError::new("Cannot add a bonus greater than 10."))
        );
    }

    #[test]
    fn test_too_many_rolls_on_frame() {
        let mut open_frame = Frame::default();
        open_frame.bowl(4).unwrap();
        open_frame.bowl(4).unwrap();

        assert_eq!(
            open_frame.bowl(1).err(),
            Some(BowlingError::new("Cannot bowl on this frame."))
        );
    }

    #[test]
    fn test_too_many_bonus_on_frame() {
        let mut open_frame = Frame::default();
        open_frame.bowl(5).unwrap();
        open_frame.bowl(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Bonus(1));
        assert_eq!(open_frame.score(), 10);

        open_frame.add_bonus(5).unwrap();
        assert_eq!(open_frame.status(), FrameStatus::Closed);
        assert_eq!(open_frame.score(), 15);

        assert_eq!(
            open_frame.add_bonus(5).err(),
            Some(BowlingError::new("Cannot add bonus points to this frame."))
        );
    }
}
