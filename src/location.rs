use crate::kgen::KMeansGeneric;
use crate::loader::Val;

#[derive(Clone, Debug, Copy, PartialOrd, PartialEq)]
pub struct Location<T> {
    pub latitude: T,
    pub longitude: T,
}

impl<T> From<&Val<T>> for Location<T>
where
    T: KMeansGeneric,
{
    fn from(report: &Val<T>) -> Self {
        Self {
            latitude: report.latitude,
            longitude: report.longitude,
        }
    }
}

impl<T> Location<T>
where
    T: KMeansGeneric,
{
    pub fn new_rand() -> Self {
        Self {
            latitude: T::rand(Some(T::from_f32(-90.0)), Some(T::from_f32(90.0))),
            longitude: T::rand(Some(T::from_f32(-180.0)), Some(T::from_f32(180.0))),
        }
    }
}
