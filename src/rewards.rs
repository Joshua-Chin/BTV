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