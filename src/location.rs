use std::hash::Hash;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

const RAD_PER_DEG: f32 = 0.017_453_292;
const RAD_PER_DEG_F64: f64 = 0.017_453_292_519_943_295;
const RAD_PER_DEG_F32: f32 = 0.017_453_292_5;
const MILES_F64: f64 = 3960.0;
const KILOMETERS_F64: f64 = 6371.0;

const KILOMETERS: f32 = 6371.0;
const MILES_F32: f32 = 3960.0;


/// a trait that is required for user data to implement
/// 
/// Example:
/// ```
/// use kmeans::location::UserDataType;
///
/// pub struct SomeUserData {
///        pub latitude: f32,
///        pub longitude: f32,
///        pub region_id: u32,
///        pub sub_region_id: u32,
///    }
///     
///    impl UserDataType<f32, u32> for SomeUserData {
///        fn get_coords<T>(&self) -> (T, T) {
///            (self.latitude, self.longitude)
///        }
///
///        fn get_unique_identifier(&self) -> (u32, u32) {
///                (self.region_id, self.sub_region_id)
///         }
///   }
///
///
///
/// ```
/// 
/// [Location] is really a link to a unique identifier K and some type T that
/// can be made into a tuple of ([T], [T]) which is the latitude and longitude
/// 
/// [T] must implement [LatLngType] and [K] must implement [Hash] and [Eq]
pub trait UserDataType<T, K>
where T: LatLngType,
        K: Hash + Eq
{
    fn get_coords<T>(&self) -> (T, T);
    fn get_unique_identifier(&self) -> K;
}

/// stores the generic bounds required for generic K throughout
pub trait LatLngType:
Copy
+ Default
+ Debug
+ PartialEq
+ PartialOrd
+ AddAssign
+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
+ Div<Output = Self>
{
    fn to_lat_lng(&self) -> (Self, Self) {
        (*self, *self)
    }
    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
    fn from_usize(value: usize) -> Self;
    fn to_usize(self) -> usize;
    fn rand(min: Option<Self>, max: Option<Self>) -> Self;
    fn is_nan(self) -> bool;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn max_value() -> Self;
    fn haversine(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self;
}

impl LatLngType for f32 {
    fn to_lat_lng(&self) -> (Self, Self) {
        (*self, *self)
    }
    fn from_f32(value: f32) -> Self {
        value
    }
    fn from_f64(value: f64) -> Self {
        value as f32
    }
    fn to_f32(self) -> f32 {
        self
    }
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_usize(value: usize) -> Self {
        value as f32
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn rand(min: Option<Self>, max: Option<Self>) -> Self {
        if let (Some(min), Some(max)) = (min, max) {
            fastrand::f32() * (max - min) + min
        } else {
            fastrand::f32()
        }
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn max_value() -> Self {
        f32::MAX
    }
    fn haversine(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        let b = |a: &Self| 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        let d_lat = (coord2.0 - coord1.0) * RAD_PER_DEG_F32;
        let d_lon = (coord2.1 - coord1.1) * RAD_PER_DEG_F32;
        let lat1 = (coord1.0) * RAD_PER_DEG_F32;
        let lat2 = (coord2.0) * RAD_PER_DEG_F32;

        let a = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
            + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
        
        b(&a) * MILES_F32
    }
}

impl LatLngType for f64 {
    fn to_lat_lng(&self) -> (Self, Self) {
        (*self, *self)
    }
    fn from_f32(value: f32) -> Self {
        value as f64
    }
    fn from_f64(value: f64) -> Self {
        value
    }
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn to_f64(self) -> f64 {
        self
    }
    fn from_usize(value: usize) -> Self {
        value as f64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn rand(min: Option<Self>, max: Option<Self>) -> Self {
        if let (Some(min), Some(max)) = (min, max) {
            fastrand::f64() * (max - min) + min
        } else {
            fastrand::f64()
        }
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn max_value() -> Self {
        f64::MAX
    }
    fn haversine(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        let b = |a: &Self| 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        let d_lat = (coord2.0 - coord1.0) * RAD_PER_DEG_F64;
        let d_lon = (coord2.1 - coord1.1) * RAD_PER_DEG_F64;
        let lat1 = (coord1.0) * RAD_PER_DEG_F64;
        let lat2 = (coord2.0) * RAD_PER_DEG_F64;

        let a = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
            + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());

        b(&a) * MILES_F64
    }

}