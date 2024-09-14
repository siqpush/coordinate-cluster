use crate::cluster::Cluster;
use crate::location::{LatLngType, UserDataType};
use std::fmt::Debug;
mod cluster;
pub mod location;
pub mod nodes;

pub fn min_max<T: LatLngType>(centroids: &[(T, T)]) -> Option<((T, T), (T, T))> {
    centroids.iter().fold(None, |acc, &(lat, lng)| match acc {
        None => Some(((lat, lat), (lng, lng))),
        Some(((lat_min, lat_max), (lng_min, lng_max))) => Some((
            (lat_min.min(lat), lat_max.max(lat)),
            (lng_min.min(lng), lng_max.max(lng)),
        )),
    })
}

pub fn calc<DATAPOINT, T>(k: usize, r: usize, data_points: &[DATAPOINT]) -> Cluster<T, DATAPOINT>
where
    DATAPOINT: UserDataType<T> + Clone + Debug,
    T: LatLngType,
{
    let mut centroids = vec![];

    for round_trip in 0..r {
        let mut cluster = Cluster::new(k, data_points, &centroids);
        let mut count_of_empty_nodes = 0;
        centroids.truncate(0);
        for cluster_node_index in 0..k {
            let node = &mut cluster.nodes[cluster_node_index];

            if !node.children.is_empty() {
                centroids.push(node.calculate_new_centroid());
            } else {
                count_of_empty_nodes += 1;
            }
        }
        // if its out last round trip and we have empty nodes, no need filling them
        if round_trip.lt(&(r - 1)) && count_of_empty_nodes > 0 {
            let lat_lng_min_max = min_max(&centroids);
            while centroids.len() < k {
                if lat_lng_min_max.is_none() {
                    centroids.push((T::rand(None, None), T::rand(None, None)));
                } else {
                    let ((lat_min, lat_max), (lng_min, lng_max)) =
                        unsafe { lat_lng_min_max.unwrap_unchecked() };
                    centroids.push((
                        T::rand(Some(lat_min), Some(lat_max)),
                        T::rand(Some(lng_min), Some(lng_max)),
                    ));
                }
            }
        }
        if round_trip.eq(&(r - 1)) {
            return cluster;
        }
    }
    unreachable!("Failed to calculate centroids")
}


#[cfg(test)]
#[allow(unused_variables)]
mod tests {

    /// sample data for testing
    #[allow(dead_code)]
    mod sample_data {
        use crate::location::UserDataType;
        
        #[derive(Clone, Debug)]
        pub struct ExampleDataPointStructF64 {
            pub lat: f64,
            pub lng: f64,
            pub region_id: i32,
            pub sub_region_id: u8,
        }

        
        #[derive(Clone, Debug)]
        pub struct ExampleDataPointStructF32 {
            pub lat: f32,
            pub lng: f32,
            pub region_id: i32,
            pub sub_region_id: u8,
        }
        impl UserDataType<f32> for ExampleDataPointStructF32 {
            fn get_coords(&self) -> (f32, f32) {
                (self.lat, self.lng)
            }
        }
        impl UserDataType<f64> for ExampleDataPointStructF64 {
            fn get_coords(&self) -> (f64, f64) {
                (self.lat, self.lng)
            }
        }

        // New York City, USA
        pub const NYC_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 40.7128,
            lng: -74.0060,
            region_id: 1,
            sub_region_id: 1,
        };

        // LONDON_F64, UK
        pub const LONDON_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 51.5074,
            lng: -0.1278,
            region_id: 2,
            sub_region_id: 1,
        };

        // TOKYO_F64, Japan
        pub const TOKYO_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 35.6762,
            lng: 139.6503,
            region_id: 3,
            sub_region_id: 1,
        };

        // SYDNEY64, Australia
        pub const SYDNEY_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: -33.8688,
            lng: 151.2093,
            region_id: 4,
            sub_region_id: 1,
        };

        // RIO_F64 de Janeiro, Brazil
        pub const RIO_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: -22.9068,
            lng: -43.1729,
            region_id: 5,
            sub_region_id: 1,
        };

        // CAIRO_F64, Egypt
        pub const CAIRO_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 30.0444,
            lng: 31.2357,
            region_id: 6,
            sub_region_id: 1,
        };

        // MOSCOW_F64, Russia
        pub const MOSCOW_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 55.7558,
            lng: 37.6173,
            region_id: 7,
            sub_region_id: 1,
        };

        // Cape Town, South Africa
        pub const CAPE_TOWN_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: -33.9249,
            lng: 18.4241,
            region_id: 8,
            sub_region_id: 1,
        };

        // MUMBAI_F64, India
        pub const MUMBAI_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 19.0760,
            lng: 72.8777,
            region_id: 9,
            sub_region_id: 1,
        };

        // North Pole
        pub const NORTH_POLE_F64: ExampleDataPointStructF64 = ExampleDataPointStructF64 {
            lat: 90.0,
            lng: 0.0,
            region_id: 10,
            sub_region_id: 1,
        };

        // New York City, USA
        pub const NYC_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 40.7128,
            lng: -74.0060,
            region_id: 1,
            sub_region_id: 1,
        };

        // LONDON_F32, UK
        pub const LONDON_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 51.5074,
            lng: -0.1278,
            region_id: 2,
            sub_region_id: 1,
        };

        // TOKYO_F32, Japan
        pub const TOKYO_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 35.6762,
            lng: 139.6503,
            region_id: 3,
            sub_region_id: 1,
        };

        // SYDNEY64, Australia
        pub const SYDNEY_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: -33.8688,
            lng: 151.2093,
            region_id: 4,
            sub_region_id: 1,
        };

        // RIO_F32 de Janeiro, Brazil
        pub const RIO_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: -22.9068,
            lng: -43.1729,
            region_id: 5,
            sub_region_id: 1,
        };

        // CAIRO_F32, Egypt
        pub const CAIRO_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 30.0444,
            lng: 31.2357,
            region_id: 6,
            sub_region_id: 1,
        };

        // MOSCOW_F32, Russia
        pub const MOSCOW_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 55.7558,
            lng: 37.6173,
            region_id: 7,
            sub_region_id: 1,
        };

        // Cape Town, South Africa
        pub const CAPE_TOWN_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: -33.9249,
            lng: 18.4241,
            region_id: 8,
            sub_region_id: 1,
        };

        // MUMBAI_F32, India
        pub const MUMBAI_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 19.0760,
            lng: 72.8777,
            region_id: 9,
            sub_region_id: 1,
        };

        // North Pole
        pub const NORTH_POLE_F32: ExampleDataPointStructF32 = ExampleDataPointStructF32 {
            lat: 90.0,
            lng: 0.0,
            region_id: 10,
            sub_region_id: 1,
        };

        pub const DATASET_F64: [ExampleDataPointStructF64; 52] = [
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            LONDON_F64,
            TOKYO_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            NYC_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
            RIO_F64,
            CAIRO_F64,
            MOSCOW_F64,
            CAPE_TOWN_F64,
            MUMBAI_F64,
            NORTH_POLE_F64,
            LONDON_F64,
            TOKYO_F64,
            SYDNEY_F64,
        ];

        pub const DATASET_F32: [ExampleDataPointStructF32; 52] = [
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            LONDON_F32,
            TOKYO_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            NYC_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
            RIO_F32,
            CAIRO_F32,
            MOSCOW_F32,
            CAPE_TOWN_F32,
            MUMBAI_F32,
            NORTH_POLE_F32,
            LONDON_F32,
            TOKYO_F32,
            SYDNEY_F32,
        ];
    }

    #[test]
    fn test_f64_data() {
        let cluster1 = super::calc(5, 1, &sample_data::DATASET_F64);
        let cluster2 = super::calc(10, 10, &sample_data::DATASET_F64);
        let total_distance1 = cluster1
            .nodes
            .iter()
            .fold(0.0, |acc, node| node.total_distance.abs());
        let total_distance2 = cluster2
            .nodes
            .iter()
            .fold(0.0, |acc, node| node.total_distance.abs());
        assert!(total_distance1 >= total_distance2);
    }

    #[test]
    fn test_f32_data() {
        for _ in 0..10 {
            let cluster1 = super::calc(5, 5, &sample_data::DATASET_F32);
            let cluster2 = super::calc(10, 10, &sample_data::DATASET_F32);
            let total_distance1 = cluster1
                .nodes
                .iter()
                .fold(0.0, |acc, node| acc + node.total_distance.abs());
            let total_distance2 = cluster2
                .nodes
                .iter()
                .fold(0.0, |acc, node| acc + node.total_distance.abs());
            assert!(total_distance1 >= total_distance2);
        }
    }
}
