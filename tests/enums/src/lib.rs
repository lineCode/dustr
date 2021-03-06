#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub enum Location {
    GPS(GPS),
    Address {
        zip: u64,
    },
    Unknown,
}

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

#[ffishim_function]
fn add_to_latitude(mut gps: GPS, lat_diff: f64) -> GPS {
    gps.lat += lat_diff;
    gps
}
