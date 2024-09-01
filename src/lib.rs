use crate::kgen::KMeansGeneric;

pub mod clusters;
pub mod distance;
mod kgen;
pub mod loader;
pub mod location;

pub fn calc_kmeans<'a, 'b, T, K>(k: usize, r: usize, values: T) -> Vec<(K, K)>
where
    T: Into<&'b [loader::Val<K>]> + Copy,
    'a: 'b,
    K: 'a + KMeansGeneric,
{
    let mut centroids = None;
    for i in 0..r {
        let mut cluster = clusters::Cluster::<K>::new(k, values.into(), &centroids);
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
            Val::new(30.0, -90.0,1),
            Val::new(20.0, -60.0,2),
            Val::new(25.0, -75.0, 3),
            // Cluster 2
            Val::new(40.0, 90.0, 4),
            Val::new(50.0, 60.0, 5),
            Val::new(45.0, 75.0, 6),
            // Cluster 3
            Val::new(-30.0, 45.0, 7),
            Val::new(-35.0, 40.0, 8),
            Val::new(-32.5, 42.5, 9),
            // Cluster 4
            Val::new(60.0, -120.0, 10),
            Val::new(65.0, -125.0, 11),
            Val::new(62.5, -122.5, 12),
        ];

        let mut correct_clusters = 0;
        for j in 0..=100 {
            let results = clusters::Cluster::calc(4, 3, &values);
            let mut correct_assignment = 0;
            for i in 1..=12 {
                if let Some((lat, lon)) = results.get(&i) {
                    if let Some((lat2, lon2)) = results.get(&(i + 1)) {
                        if i.eq(&3) || i.eq(&6) || i.eq(&9) {
                            if lat.ne(&lat2) && lon.ne(&lon2) {
                                correct_assignment += 1;
                            }
                        } else if lat.eq(&lat2) && lon.eq(&lon2) {
                            correct_assignment += 1;
                        }
                    }
                }
            }
            if correct_assignment > 9 {
                correct_clusters += 1;
            }
        }
        println!("Correct clusters: {}", correct_clusters);
        assert!(correct_clusters>60);
    }
}
