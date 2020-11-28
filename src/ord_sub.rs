use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
pub struct OrdSub<T: PartialOrd>(T);

impl OrdSub<f32> {
    pub fn new(val: f32) -> Option<OrdSub<f32>> {
        if val.is_nan() {
            None
        } else {
            Some(OrdSub(val))
        }
    }
}

impl<T: PartialOrd> Eq for OrdSub<T> {}

impl<T: PartialOrd> Ord for OrdSub<T> {
    fn cmp(&self, other: &OrdSub<T>) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}