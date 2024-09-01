#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Val<T> {
    pub latitude: T,
    pub longitude: T,
    pub id: i32,
}

impl<T> Val<T>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new(lat: T, lon: T) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
            id: 0,
        }
    }
}
