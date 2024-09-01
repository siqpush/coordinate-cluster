use crate::distance::{haversine_f32, haversine_f64, Units};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

pub trait KMeansGeneric:
    Copy
    + Default
    + Debug
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
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
    fn haversine_miles(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self;
    fn haversine_km(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self;
}

impl KMeansGeneric for f32 {
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
    fn haversine_miles(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        haversine_f32(coord1, coord2, Units::Miles)
    }
    fn haversine_km(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        haversine_f32(coord1, coord2, Units::Kilometers)
    }
}

impl KMeansGeneric for f64 {
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
    fn haversine_miles(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        haversine_f64(coord1, coord2, Units::Miles)
    }
    fn haversine_km(coord1: &(Self, Self), coord2: &(Self, Self)) -> Self {
        haversine_f64(coord1, coord2, Units::Kilometers)
    }
}
