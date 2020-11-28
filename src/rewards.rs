use std::str::FromStr;

bitflags! {
    pub struct Rewards: u32 {
        const NONE = 0;
        const ADDITIONAL_ABILITY = 1 << 0;
        const ATMOSPHERE_RANGE = 1 << 1;
        const DICTION_STRENGTH = 1 << 2;
        const DICTION_RANGE = 1 << 3;
        const PRECISION_STRENGTH = 1 << 4;
        const PRECISION_RANGE = 1 << 5;
        const CALMNESS_STRENGTH = 1 << 6;
        const STYLE_EXPLODING = 1 << 7;
    }
}

impl Rewards {
    pub fn combinations() -> impl Iterator<Item = Rewards> {
        (0..256).map(Rewards::from_bits_truncate)
    }
}

impl FromStr for Rewards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+1 Ability max per challenge" => Rewards::ADDITIONAL_ABILITY,
            "+1 Atmosphere Range" => Rewards::ATMOSPHERE_RANGE,
            "+1 Diction Strength" => Rewards::DICTION_STRENGTH,
            "+1 Diction Range" => Rewards::DICTION_RANGE,
            "+1 Precision Strength" => Rewards::DICTION_STRENGTH,
            "+1 Precision Range" => Rewards::PRECISION_RANGE,
            "+1 Calmness Strength" => Rewards::CALMNESS_STRENGTH,
            "+1 Style Attempt on Style roll of 19 or 20" => Rewards::STYLE_EXPLODING,
            _ => Rewards::NONE
        })
    }
}