use crate::nodes::Node;
use crate::user_data::{LatLngType, UserDataType};

pub struct Cluster<T, DATAPOINT>
where
    T: LatLngType,
    DATAPOINT: UserDataType<T> + Clone,
{
    pub nodes: Vec<Node<T, DATAPOINT>>,
}

impl<T, DATAPOINT> Cluster<T, DATAPOINT>
where
    T: LatLngType,
    DATAPOINT: UserDataType<T> + Clone,
{
    pub fn new(k: usize, data_points: &[DATAPOINT], centroids: &[(T, T)]) -> Self {
        let mut cluster = Self { nodes: vec![] };
        if !centroids.is_empty() {
            cluster.assign_centroids_to_nodes(centroids);
        } else {
            cluster.initialize_first_cluster(k, data_points);
        }

        // Assign the reports to the closest node
        for data_point in data_points.iter() {
            // determine the closest node to the report
            let closest_node_and_distance_tuple =
                cluster.assign_data_point_to_closest_node(data_point);
            cluster.nodes[closest_node_and_distance_tuple.0].push_child(data_point);
            cluster.nodes[closest_node_and_distance_tuple.0].total_distance +=
                closest_node_and_distance_tuple.1;
        }
        cluster
    }

    /// for one location determine the closest node (k node) to the location
    /// return the index of the node and the distance to the node
    pub fn assign_data_point_to_closest_node(&self, data_point: &DATAPOINT) -> (usize, T) {
        // tuple of k node and the distance to the node
        let mut min_loc: (usize, T) = (usize::default(), T::max_value());

        // if the distance to the node is less than the current minimum distance, update the minimum distance
        for (k, node) in self.nodes.iter().enumerate() {
            let dist = T::haversine(&node.location, &data_point.get_coords());
            if dist < min_loc.1 {
                min_loc = (k, dist);
            }
        }
        min_loc
    }

    /// if provided with centroid values assign the lat lngs to the clusters nodes
    fn assign_centroids_to_nodes(&mut self, centroids: &[(T, T)]) {
        for centroid in centroids {
            self.nodes.push(Node::new(centroid.0, centroid.1, vec![]));
        }
    }

    /// Assign k nodes to the cluster
    fn initialize_first_cluster(&mut self, k: usize, data_points: &[DATAPOINT]) {
        let mut min_lat = None;
        let mut max_lat = None;
        let mut min_lon = None;
        let mut max_lon = None;
        for data_point in data_points {
            let data_point_coords = data_point.get_coords();
            if min_lat.map(|ml| data_point_coords.0 > ml).unwrap_or(true) {
                min_lat = Some(data_point_coords.0);
            }
            if max_lat.map(|ml| data_point_coords.0 < ml).unwrap_or(true) {
                max_lat = Some(data_point_coords.0);
            }
            if min_lon.map(|ml| data_point_coords.1 > ml).unwrap_or(true) {
                min_lon = Some(data_point_coords.1);
            }
            if max_lon.map(|ml| data_point_coords.1 < ml).unwrap_or(true) {
                max_lon = Some(data_point_coords.1);
            }
        }

        for _ in 0..=k {
            self.nodes.push(Node::new(
                T::rand(min_lat, max_lat),
                T::rand(min_lon, max_lon),
                vec![],
            ));
        }
    }
}
