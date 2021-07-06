use std::ops::{Index, IndexMut};

/// Enumeration of available abilities.
#[derive(Debug, Copy, Clone)]
pub enum Ability {
    Atmosphere,
    Diction,
    Precision,
    Calmness,
    Focus,
    Style,
    Rhythm,
    Timing,
}

impl Ability {
    /// Returns an array of possible abilities.
    pub const fn values() -> [Ability; 8] {
        [
            Ability::Atmosphere,
            Ability::Diction,
            Ability::Precision,
            Ability::Calmness,
            Ability::Focus,
            Ability::Style,
            Ability::Rhythm,
            Ability::Timing,
        ]
    }

    /// Returns the cost of the ability.
    pub const fn cost(&self) -> u32 {
        match self {
            Ability::Atmosphere => 4,
            Ability::Diction => 6,
            Ability::Precision => 8,
            Ability::Calmness => 10,
            Ability::Focus => 12,
            Ability::Style => 20,
            Ability::Rhythm => 30,
            Ability::Timing => 100,
        }
    }
}

/// Efficient ability counter.
#[derive(Copy, Clone, Debug, Default)]
pub struct Abilities {
    abilities: [u8; 8],
}

impl Abilities {
    pub const fn new() -> Abilities {
        Abilities { abilities: [0; 8] }
    }

    const fn index_of(ability: Ability) -> usize {
        match ability {
            Ability::Atmosphere => 0,
            Ability::Diction => 1,
            Ability::Precision => 2,
            Ability::Calmness => 3,
            Ability::Focus => 4,
            Ability::Style => 5,
            Ability::Rhythm => 6,
            Ability::Timing => 7,
        }
    }
}

impl Index<Ability> for Abilities {
    type Output = u8;

    fn index(&self, index: Ability) -> &Self::Output {
        &self.abilities[Self::index_of(index)]
    }
}

impl IndexMut<Ability> for Abilities {
    fn index_mut(&mut self, index: Ability) -> &mut Self::Output {
        &mut self.abilities[Self::index_of(index)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_set() {
        let mut ability_set = Abilities::new();
        assert_eq!(ability_set[Ability::Diction], 0);
        ability_set[Ability::Diction] = 4;
        assert_eq!(ability_set[Ability::Diction], 4);
    }
}
