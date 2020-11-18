const LIMIT: usize = 71;

// A distribution representing the sum of multiple rolls.
#[derive(Clone, Debug)]
pub struct Dist {
    // The complementary cumulative distribution
    ccdf: [f32; LIMIT],
}

impl Dist {
    pub fn new() -> Dist {
        let mut ccdf = [0.0; LIMIT];
        ccdf[0] = 1.0;
        Dist { ccdf }
    }

    pub fn at_least(&self, target: usize) -> f32 {
        self.ccdf[target]
    }

    pub fn add_die(&self, range: u32, strength: bool) -> Dist {
        let mut output = Dist::new();
        let mut window = range as f32;
        let range_us = range as usize;
        
        if strength {
            for i in 1..LIMIT {
                output.ccdf[i] = window / (range as f32);
                window += self.ccdf[i - 1];
                window += if i + 1 >= range_us { self.ccdf[i + 1 - range_us] } else { 1.0 };
                window -= 2.0 * if i >= range_us { self.ccdf[i - range_us] } else { 1.0 };
            }
        } else {
            for i in 1..LIMIT {
                output.ccdf[i] = window / (range as f32);
                window += self.ccdf[i];
                window -= if i >= range_us { self.ccdf[i - range_us] } else { 1.0 };
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dist = Dist::new();
        assert_eq!(dist.at_least(0), 1.0);
        assert_eq!(dist.at_least(1), 0.0);
    }

    #[test]
    fn test_add_die() {
        let dist = Dist::new().add_die(8, false);
        assert_eq!(dist.at_least(1), 1.0);
        assert_eq!(dist.at_least(2), 0.875);
        assert_eq!(dist.at_least(8), 0.125);
        assert_eq!(dist.at_least(9), 0.0);
    }

    #[test]
    fn test_add_strength_die() {
        let dist = Dist::new().add_die(8, true);
        assert_eq!(dist.at_least(2), 1.0);
        assert_eq!(dist.at_least(3), 0.875);
        assert_eq!(dist.at_least(8), 0.25);
        assert_eq!(dist.at_least(9), 0.0);
    }
}