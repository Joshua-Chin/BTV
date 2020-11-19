enum Abilities {
    Atmosphere,
    Diction,
    Precision,
    Calmness,
    Focus,
    Style,
    Rhythm,
    Timing,
}

impl Abilities {
    pub fn cost(&self) -> u32 {
        match self {
            Abilities::Atmosphere => 4,
            Abilities::Diction => 6,
            Abilities::Precision => 8,
            Abilities::Calmness => 10,
            Abilities::Focus => 12,
            Abilities::Style => 20,
            Abilities::Rhythm => 30,
            Abilities::Timing => 100,
        }
    }
}