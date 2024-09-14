use std::hash::Hash;
use crate::location::{LatLngType, UserDataType};

/// a node holds a centroid location and a list of user provided data points
/// it also marks the total distance of the children to the centroid
/// 
/// [DATAPOINT] must implement [UserDataType] and [T] must implement [LatLngType]
#[derive(Clone, Debug)]
pub struct Node<T, DATAPOINT>
where T: LatLngType, DATAPOINT: UserDataType<T> + Clone
{
    pub location: (T, T),
    pub children: Vec<DATAPOINT>,
    pub total_distance: T,
}

impl<T, DATAPOINT> Node<T, DATAPOINT>
where T: LatLngType, DATAPOINT: UserDataType<T> + Clone
{
    pub fn new(lat: T, lng: T, children: Vec<DATAPOINT>) -> Self {
        Self {
            location: (lat, lng),
            children,
            total_distance: T::default(),
        }
    }

    pub fn push_child(&mut self, data_point: &DATAPOINT) {
        self.children.push(data_point.clone());
    }

    pub fn calculate_new_centroid(&mut self) -> (T, T) {
        let (sum_lat, sum_lon) = self
            .children
            .iter()
            .map(|child| child.get_coords())
            .fold(
                (T::from_usize(0), T::from_usize(0)),
                |(acc_lat, acc_lon), (lat, lon)| (acc_lat + lat, acc_lon + lon),
            );

        let count = T::from_usize(self.children.len());

        let mean_lat = sum_lat / count;
        let mean_lon = sum_lon / count;

        (mean_lat, mean_lon)
    }

    pub fn has_only_unique_pairs(&self) -> bool {
        for i in self.children.iter() {
            for j in self.children.iter() {
                if i.get_coords() != j.get_coords() {
                    return false;
                }
            }
        }
        true
    }
}
