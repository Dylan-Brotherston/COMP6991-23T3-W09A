#![allow(dead_code)]

use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum TimeOfDay {
    Morning,
    Midday,
    Evening,
    Midnight,
    Total,
}

#[derive(Debug)]
struct Entry {
    time_period: String,
    station: String,
    entries: HashMap<TimeOfDay, i32>,
    exits: HashMap<TimeOfDay, i32>,
    latitude: f64,
    longitude: f64,
}
#[derive(Deserialize, Debug)]
struct CSVEntry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn convert_csventry_to_entry(csv_entry: &CSVEntry) -> Entry {
    let mut entry = Entry {
        time_period: csv_entry.time_period.clone(),
        station: csv_entry.station.clone(),
        entries: HashMap::new(),
        exits: HashMap::new(),
        latitude: csv_entry.latitude,
        longitude: csv_entry.longitude,
    };

    if let Some(e) = csv_entry.entries_morning {
        entry.entries.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_entry.entries_midday {
        entry.entries.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_entry.entries_evening {
        entry.entries.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_entry.entries_midnight {
        entry.entries.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_entry.entries_total {
        entry.entries.insert(TimeOfDay::Total, e);
    }

    if let Some(e) = csv_entry.exits_morning {
        entry.exits.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_entry.exits_midday {
        entry.exits.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_entry.exits_evening {
        entry.exits.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_entry.exits_midnight {
        entry.exits.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_entry.exits_total {
        entry.exits.insert(TimeOfDay::Total, e);
    }

    entry
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let entries: Vec<CSVEntry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    let entries: Vec<Entry> = entries
        .into_iter()
        .map(|ent| convert_csventry_to_entry(&ent))
        .collect();

    let most = most_used_station(&entries).expect("Should be some max");
    let least = least_used_station(&entries).expect("Should be some min");
    println!("Most Used: {most:?}");
    println!(
        "Busiest time of day {:?}",
        get_busiest_times_of_day(&entries, &most)
    );
    println!(
        "Busiest time of year {:?}",
        get_busiest_year(&entries, &most)
    );
    println!("Least Used: {least:?}");
    println!(
        "Busiest time of day {:?}",
        get_busiest_times_of_day(&entries, &least)
    );
    println!(
        "Busiest time of year {:?}",
        get_busiest_year(&entries, &least)
    );

    Ok(())
}

fn get_station_traffic(entries: &[Entry]) -> HashMap<&String, i32> {
    let mut hm: HashMap<&String, i32> = HashMap::new();
    for ent in entries {
        // Calculate total entries and exits for the station
        let station_total: i32 = ent.entries.get(&TimeOfDay::Total).unwrap_or(&0)
            + ent.exits.get(&TimeOfDay::Total).unwrap_or(&0);

        // Add the total to the hashmap
        *hm.entry(&ent.station).or_insert(0) += station_total;
    }
    hm
}

fn most_used_station(entries: &[Entry]) -> Option<String> {
    let hm = get_station_traffic(entries);
    hm.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(station, _)| station.clone())
}
fn least_used_station(entries: &[Entry]) -> Option<String> {
    let hm = get_station_traffic(entries);
    hm.into_iter()
        .min_by_key(|(_, count)| *count)
        .map(|(station, _)| station.clone())
}

fn get_busiest_times_of_day(entries: &[Entry], station: &str) -> Option<TimeOfDay> {
    let station_entries: Vec<&Entry> = entries.iter().filter(|e| e.station == station).collect();

    if station_entries.is_empty() {
        return None;
    }

    let mut time_counts: HashMap<TimeOfDay, i32> = HashMap::new();
    for ent in station_entries {
        for (time_of_day, count) in &ent.entries {
            if time_of_day != &TimeOfDay::Total {
                *time_counts.entry(*time_of_day).or_insert(0) += *count;
            }
        }
    }

    time_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(time_of_day, _)| time_of_day)
}

// Function to get the busiest year for a given station
fn get_busiest_year(entries: &[Entry], station: &str) -> Option<String> {
    let station_entries: Vec<&Entry> = entries.iter().filter(|e| e.station == station).collect();

    if station_entries.is_empty() {
        return None;
    }

    let mut year_counts: HashMap<String, i32> = HashMap::new();
    for ent in &station_entries {
        *year_counts.entry(ent.time_period.clone()).or_insert(0) +=
            ent.entries.get(&TimeOfDay::Total).unwrap_or(&0)
                + ent.exits.get(&TimeOfDay::Total).unwrap_or(&0);
    }

    year_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(year, _)| year)
}
