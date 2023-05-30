use chrono::prelude::{DateTime, Local};
use std::thread;
use std::time::Duration;
use std::{fs, process};
use time;

use log::debug;
use log::LevelFilter;
use monsoon::Monsoon;
use serde::{Deserialize, Serialize};
use toml;

#[tokio::main]
async fn main() {
    simple_logging::log_to_stderr(LevelFilter::Debug);

    let locations = read_route_db();

    let monsoon = Monsoon::new("rainy dawid@ricsys.co.za").expect("Error at monsoon");

    for location in locations.0 {
        let response = monsoon
            .get(location.latitude, location.longitude)
            .await
            .expect("Cannot get location");

        let body = response.body().expect("Cannot get body");

        println!("Point {:?}", body.geometry);
        println!("Last Update {}", body.properties.meta.updated_at);

        thread::sleep(Duration::from_millis(200));
    }
}

fn read_route_db() -> LocationConfig {
    let locations = fs::read_to_string("config/loc.json").expect("Cannot read Locations file!!!");

    let deserialised: LocationConfig =
        serde_json::from_str(&locations).expect("Cannot deserialise locations!");

    debug!("{:?}", deserialised);

    deserialised
}

#[derive(Serialize, Deserialize, Debug)]
struct LocationConfig(Vec<Location>);

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    latitude: f64,
    longitude: f64,
}

struct WeatherData {
    update_time: DateTime<Local>,
    location_readings: Vec<WeatherReading>,
}

struct WeatherReading {
    updated_at: DateTime<Local>,
    latitude: f64,
    longitude: f64,

    air_pressure: Option<f64>,
    temperature: Option<f64>,
    cloud_area_fraction: Option<f64>,
    cloud_area_low: Option<f64>,
    cloud_area_high: Option<f64>,
    dew_point_temperatur: Option<f64>,
    fog_area: Option<f64>,
    relative_humidity: Option<f64>,

    next_hour_precip_min: Option<f64>,
    next_hour_precip_max: Option<f64>,
    next_hour_precip_chance: Option<f64>,
}
impl WeatherReading {}
