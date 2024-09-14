const RAD_PER_DEG: f32 = 0.017_453_292;
const RAD_PER_DEG_F64: f64 = 0.017_453_292_519_943_295;
const MILES_F64: f64 = 3960.0;
const KILOMETERS_F64: f64 = 6371.0;

const KILOMETERS: f32 = 6371.0;
const MILES: f32 = 3960.0;

pub enum Units {
    Miles,
    Kilometers,
}

pub fn haversine_f32(start: &(f32, f32), end: &(f32, f32), units: Units) -> f32 {
    let b = |a: &f32| 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let d_lat = (end.0 - start.0) * RAD_PER_DEG;
    let d_lon = (end.1 - start.1) * RAD_PER_DEG;
    let lat1 = (start.0) * RAD_PER_DEG;
    let lat2 = (end.0) * RAD_PER_DEG;

    let a = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());

    match units {
        Units::Miles => b(&a) * MILES,
        Units::Kilometers => b(&a) * KILOMETERS,
    }
}


