use crate::kgen::KMeansGeneric;
use crate::loader::Val;
use crate::location::Location;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub location: Location<T>,
    pub children: Vec<Val<T>>,
    pub total_distance: T,
}

impl<T> Node<T>
where
    T: KMeansGeneric,
{
    pub fn new(lat: Option<T>, lon: Option<T>) -> Self {
        if lat.is_none() || lon.is_none() {
            Self {
                location: Location::new_rand(),
                children: vec![],
                total_distance: T::from_usize(0),
            }
        } else {
            Self {
                location: Location {
                    latitude: unsafe { lat.unwrap_unchecked() },
                    longitude: unsafe { lon.unwrap_unchecked() },
                },
                children: vec![],
                total_distance: T::from_usize(0),
            }
        }
    }

    pub fn push_child(&mut self, report: Val<T>) {
        self.children.push(report);
    }

    pub fn calculate_new_centroid(&mut self) -> (T, T) {
        let (sum_lat, sum_lon) = self
            .children
            .iter()
            .map(|child| (child.latitude, child.longitude))
            .fold(
                (T::from_usize(0), T::from_usize(0)),
                |(acc_lat, acc_lon), (lat, lon)| (acc_lat + lat, acc_lon + lon),
            );

        let count = T::from_usize(self.children.len());

        let mean_lat = sum_lat / count;
        let mean_lon = sum_lon / count;

        (mean_lat, mean_lon)
    }
}

#[derive(Default)]
pub struct Cluster<T> {
    pub nodes: HashMap<usize, Node<T>>,
}

impl<T> Cluster<T>
where
    T: KMeansGeneric,
{
    pub fn new(k: usize, reports: &[Val<T>], assigned_nodes: &Option<Vec<(T, T)>>) -> Self {
        let mut c = Self::default();
        if let Some(nodes) = assigned_nodes {
            for (i, node) in nodes.iter().enumerate() {
                c.nodes.insert(i, Node::new(Some(node.0), Some(node.1)));
            }
        } else {
            let mut rng = fastrand::Rng::new();
            let mut selected_indices = vec![];
            for _ in 0..k {
                selected_indices.push(rng.usize(..reports.len()));
            }

            for (i, &index) in selected_indices.iter().enumerate() {
                c.nodes.insert(
                    i,
                    Node::new(
                        Some(reports[index].latitude),
                        Some(reports[index].longitude),
                    ),
                );
            }
        }

        for (i, report) in reports.iter().enumerate() {
            let mut min_loc: (usize, i32, T) = (0, 0, T::max_value());
            for (k, node) in c.nodes.iter() {
                let dist = T::haversine_miles(
                    &(node.location.latitude, node.location.longitude),
                    &(report.latitude, report.longitude),
                );
                if dist < min_loc.2 {
                    min_loc = (*k, report.id, dist);
                }
            }
            c.nodes
                .entry(min_loc.0)
                .and_modify(|node| node.push_child(reports[i]));
        }
        c
    }

    pub fn calc(k: usize, r: usize, values: &[Val<T>]) -> (T, Vec<(T, T)>) {
        let mut centroids = None;
        for i in 0..r {
            let mut cluster = Self::new(k, values, &centroids);
            // Clear the centroids since we already defined them and we dont want the next round to push onto the existing centroids
            if let Some(ref mut centroids) = centroids {
                centroids.clear();
            } else {
                centroids = Some(vec![]);
            }
            let mut total_distance = T::default();
            for (_, node) in cluster.nodes.iter_mut() {
                for report in node.children.iter() {
                    node.total_distance += T::haversine_miles(
                        &(node.location.latitude, node.location.longitude),
                        &(report.latitude, report.longitude),
                    );
                }
                total_distance += node.total_distance;
                if let Some(ref mut centroids) = centroids {
                    let centroid = node.calculate_new_centroid();
                    centroids.push(centroid);
                }
            }
            if i.eq(&(r - 1)) {
                return (total_distance, centroids.unwrap());
            }
        }
        unreachable!("Failed to calculate centroids")
    }
}
