use std::hash::Hash;
use crate::location::{LatLngType, UserDataType};
use crate::nodes::Node;

pub struct Cluster<T, K, DATAPOINT>{
    pub nodes: Vec<Node<T, K, DATAPOINT>>,
}

impl<T, K, DATAPOINT> Cluster<T, K, DATAPOINT>
where T: LatLngType, DATAPOINT: UserDataType<T, K>, K: Hash + Eq
{

    fn new(k: usize, data_points: Vec<DATAPOINT>, assigned_nodes: &Option<Vec<(T, T)>>) -> Self {
        let mut cluster = Self {
            nodes: vec![],
        };
        if let Some(nodes) = assigned_nodes {
            cluster.assign_nodes(nodes.clone());
        } else {
            cluster.assign_locations_to_randomly_chosen_node(k, data_points);
        }

        // Assign the reports to the closest node
        for (i, report) in data_points.iter().enumerate() {
            // determine the closest node to the report
            let closest_node_and_distance_tuple = cluster.closest_node(&data_points[i]);

            cluster.nodes[i].push_child(data_points[i]);
        }
        cluster
    }


    /// for one location determine the closest node (k node) to the location
    /// return the index of the node and the distance to the node
    pub fn closest_node(&self, report: &Val<T, K>) -> (usize, T) {
        // tuple of k node and the distance to the node
        let mut min_loc: (usize, T) = (usize::default(), T::max_value());

        // if the distance to the node is less than the current minimum distance, update the minimum distance
        for (k, node) in self.nodes.iter() {
            let dist = T::haversine_miles(
                &(node.get_coords()),
                &(report.latitude, report.longitude),
            );
            if dist < min_loc.1 {
                min_loc = (*k, dist);
            }
        }
        min_loc
    }

    fn assign_nodes(&mut self, nodes: Vec<(T, T)>)
    where Self: Into<(T, T)>
    {
        for (i, node) in nodes.iter().enumerate() {
            self.nodes.insert(i, Node::new(Some(node.0), Some(node.1)));
        }
    }

    /// Assign k nodes to the cluster
    pub fn assign_locations_to_randomly_chosen_node(&mut self, k: usize, locations: &[Val<T, K>]) {
        // Randomly select k nodes from the reports
        let mut rng = fastrand::Rng::new();
        let mut selected_indices = vec![];
        for _ in 0..k {
            selected_indices.push(rng.usize(..locations.len()));
        }

        // Assign the selected nodes to the cluster
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

    /// take a list of values and return unique id and the location of the centroid
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
