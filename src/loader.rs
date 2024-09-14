#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Val<T, K> {
    pub latitude: T,
    pub longitude: T,
    pub unique_identifier: K,
}

impl<T, K> Val<T, K>
where
    T: Copy + std::fmt::Debug,
{
    pub fn new(lat: T, lon: T, unique_identifier: K) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
            unique_identifier,
        }
    }
    
    pub fn get_coords(&self) -> (T, T) {
        (self.latitude, self.longitude)
    }
}
