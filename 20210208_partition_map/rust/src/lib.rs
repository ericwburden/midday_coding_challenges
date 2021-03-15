//! MID DAY CODING CHALLENGE: Write the code in any language you like, and do
//! not worry if anybody has solved it before you!
//! RIGHT PLACEMENT:
//! Given a map / dict of {Partition: Memory}, and a memory value, return the
//! optimal partition name for the memory given (job). If no partitions satisfy
//! the memory requirements, return a scary error message and exit.
//!
//! ```ignore
//! Partition:
//! {'XSMALL': 1000,
//!  'SMALL': 4000,
//!  'MEDONE': 8000,
//!  'MEDTWO': 16000,
//!  'MEDTHREE': 24000,
//!  'LARGEONE': 32000,
//!  'LARGETWO': 54000,
//!  'LARGETHREE': 64000,
//!  'XLARGE': 128000,}
//! ```
//!
//! Clarifications:
//! You will need to create your own partition map to use, but the numbers should
//! be the ones from the above example!
//!
//! If a job has a memory requirement of 2000, you should place it in SMALL,
//! because this is optimal:
//! XSMALL < 2000 < SMALL
//!
//! Example data:
//!
//! ```ignore
//! find_partition_by_memory(partition_map, 900)    -> XSMALL
//! find_partition_by_memory(partition_map, 2000)   -> SMALL
//! find_partition_by_memory(partition_map, 6000)   -> MEDONE
//! find_partition_by_memory(partition_map, 10000)  -> MEDTWO
//! find_partition_by_memory(partition_map, 20000)  -> MEDTHREE
//! find_partition_by_memory(partition_map, 30000)  -> LARGEONE
//! find_partition_by_memory(partition_map, 40000)  -> LARGETWO
//! find_partition_by_memory(partition_map, 60000)  -> LARGETHREE
//! find_partition_by_memory(partition_map, 100000) -> XLARGE
//! find_partition_by_memory(partition_map, 200000) -> "ABORT! THIS MUCH MEMORY IS NOT AVAILABLE!"
//! ```
//!
//! Good luck and have fun!

/// Struct that holds partition name and size information
pub struct PartitionSize<'a> {
    name: &'a str,
    min: u32,
    max: u32,
}

impl<'a> PartitionSize<'a> {
    /// Create a new PartitionSize of given `name`, minimum value `min`, and
    /// maximum value `max`
    pub const fn new(name: &'a str, min: u32, max: u32) -> Self {
        PartitionSize {
            name: name,
            min,
            max,
        }
    }

    /// Prepares a constant mapping of partition names to sizes.
    #[rustfmt::skip]
    pub const fn build_map() -> [Self; 9] {
        [
            PartitionSize::new(    "XSMALL",      0,   1_000),
            PartitionSize::new(     "SMALL",  1_000,   4_000),
            PartitionSize::new(    "MEDONE",  4_000,   8_000),
            PartitionSize::new(    "MEDTWO",  8_000,  16_000),
            PartitionSize::new(  "MEDTHREE", 16_000,  24_000),
            PartitionSize::new(  "LARGEONE", 24_000,  32_000),
            PartitionSize::new(  "LARGETWO", 32_000,  54_000),
            PartitionSize::new("LARGETHREE", 54_000,  64_000),
            PartitionSize::new(    "XLARGE", 64_000, 128_000),
        ]
    }

    /// Tests a number to see if it fits in this partition
    pub fn fits(&self, n: u32) -> bool {
        if self.min < n && n <= self.max {
            true
        } else {
            false
        }
    }
}

pub mod partition {
    use crate::PartitionSize;

    // Builds PARTITION_MAP at compile time and makes it available to any other
    // code in the `partition` module.
    const PARTITION_MAP: [PartitionSize; 9] = PartitionSize::build_map();

    /// For a given number `n`, return the name of the partition it fits into,
    /// if one can be found. Error otherwise.
    pub fn find_partition_by_memory<'b>(n: u32) -> Result<&'b str, &'static str> {
        for partition in &PARTITION_MAP {
            if partition.fits(n) {
                return Ok(partition.name);
            }
        }
        Err("Segfault. Good luck with that.")
    }
}

#[cfg(test)]
mod tests {
    use super::partition::find_partition_by_memory;

    #[test]
    #[rustfmt::skip]
    fn it_works() {
        assert_eq!(find_partition_by_memory(900),     Ok("XSMALL"));
        assert_eq!(find_partition_by_memory(2_000),   Ok("SMALL"));
        assert_eq!(find_partition_by_memory(6_000),   Ok("MEDONE"));
        assert_eq!(find_partition_by_memory(10_000),  Ok("MEDTWO"));
        assert_eq!(find_partition_by_memory(20_000),  Ok("MEDTHREE"));
        assert_eq!(find_partition_by_memory(30_000),  Ok("LARGEONE"));
        assert_eq!(find_partition_by_memory(40_000),  Ok("LARGETWO"));
        assert_eq!(find_partition_by_memory(60_000),  Ok("LARGETHREE"));
        assert_eq!(find_partition_by_memory(100_000), Ok("XLARGE"));

        assert!(find_partition_by_memory(200_000).is_err());
    }
}
