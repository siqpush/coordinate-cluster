use std::collections::HashMap;
use std::sync::Mutex;
use fastrand::u128;
use lazy_static::lazy_static;
use crate::location::ProvidedLocation;
use crate::nodes::ClusteredVal;
use crate::kgen::KMeansGeneric;
use crate::loader::Val;

pub mod nodes;
pub mod distance;
pub mod kgen;
pub mod loader;
pub mod location;
mod cluster;

use once_cell::sync::OnceCell;

pub fn init

pub fn calc_kmeans<'a, 'b, T, K>(k: usize, r: usize, values: T) -> Vec<Val<T, K>>
where
    T: Into<&'b [Val<T, K>]> + Copy,
    'a: 'b,
    K: 'a + KMeansGeneric,
{
    let mut centroids = None;
    for i in 0..r {
        let mut cluster = nodes::Cluster::<K>::new(k, values.into(), &centroids);
        // Clear the centroids since we already defined them and we dont want the next round to push onto the existing centroids
        if let Some(ref mut centroids) = centroids {
            centroids.clear();
        } else {
            centroids = Some(vec![]);
        }
        for (_, node) in cluster.nodes.iter_mut() {
            for report in node.children.iter() {
                node.total_distance += K::haversine_miles(
                    &(node.location.latitude, node.location.longitude),
                    &(report.latitude, report.longitude),
                );
            }
            if let Some(ref mut centroids) = centroids {
                centroids.push(ClusteredVal::new(node.calculate_new_centroid()));
                centroids.push(node.calculate_new_centroid());
            }
        }
        if i.eq(&(r - 1)) {
            return centroids.unwrap();
        }
    }
    unreachable!("Failed to calculate centroids")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::Val;

    #[test]
    fn test_calc_kmeans() {
        let values: Vec<Val<f64>> = vec![
            // Cluster 1
            Val::new(30.0, -90.0),
            Val::new(20.0, -60.0),
            Val::new(25.0, -75.0),
            // Cluster 2
            Val::new(40.0, 90.0),
            Val::new(50.0, 60.0),
            Val::new(45.0, 75.0),
            // Cluster 3
            Val::new(-30.0, 45.0),
            Val::new(-35.0, 40.0),
            Val::new(-32.5, 42.5),
            // Cluster 4
            Val::new(60.0, -120.0),
            Val::new(65.0, -125.0),
            Val::new(62.5, -122.5),
        ];
        let (distance2, _results2) = nodes::Cluster::calc(2, 3, &values);
        let (distance4, _results4) = nodes::Cluster::calc(4, 3, &values);
        assert!(distance4 < distance2);
    }
}
