use std::time::Duration;

use aprs_encode::{
    ddm::{CardinalDirection, DdmLatitude, DdmLongitude, DegreeMinutes},
    stack_str::PackArrayString,
};
use arrayvec::ArrayString;
use callpass::Callpass;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("callsign")
                .takes_value(true)
                .help("Your callsign")
                .required(true),
        )
        .arg(
            Arg::with_name("latitude")
                .long("latitude")
                .takes_value(true)
                .help("Latitude")
                .required(true),
        )
        .arg(
            Arg::with_name("longitude")
                .long("longitude")
                .takes_value(true)
                .help("Longitude")
                .required(true),
        )
        .arg(
            Arg::with_name("message")
                .long("message")
                .short("m")
                .takes_value(true)
                .help("Message to broadcast")
                .required(false),
        )
        .arg(
            Arg::with_name("symbol")
                .long("symbol")
                .short("s")
                .takes_value(true)
                .help("APRS symbol")
                .default_value("-")
                .required(false),
        )
        .get_matches();

    // Get data
    let callsign = matches.value_of("callsign").unwrap().to_uppercase();
    let callpass: Callpass = callsign.clone().into();
    let symbol = matches.value_of("symbol").unwrap();
    let message = matches.value_of("message");
    let latitude: f32 = matches.value_of("latitude").unwrap().parse().unwrap();
    let longitude: f32 = matches.value_of("longitude").unwrap().parse().unwrap();

    // Calculate position in DDM
    let mut ddm_longitude = ArrayString::<128>::new();
    DdmLongitude {
        ddm: DegreeMinutes::from(longitude.abs()),
        direction: if longitude >= 0.0 {
            CardinalDirection::East
        } else {
            CardinalDirection::West
        },
    }
    .pack_into(&mut ddm_longitude)
    .unwrap();
    let mut ddm_latitude = ArrayString::<128>::new();
    DdmLatitude {
        ddm: DegreeMinutes::from(latitude.abs()),
        direction: if latitude >= 0.0 {
            CardinalDirection::North
        } else {
            CardinalDirection::South
        },
    }
    .pack_into(&mut ddm_latitude)
    .unwrap();

    // Encode the packet
    let encoded_data = format!(
        "{}>{}:!{}/{}{} {}",
        callsign,
        "APRS",
        ddm_latitude,
        ddm_longitude,
        symbol,
        message.unwrap_or("")
    );

    // Print the packet
    println!("-> {}", encoded_data);

    // Set up the HTTP client
    let client = reqwest::Client::new();
    let response = client
        .post("http://rotate.aprs.net:8080/")
        .header("Accept-Type", "text/plain")
        .header("Content-Type", "application/octet-stream")
        .body(format!(
            "user {} pass {} vers aprshttp {}\n{}",
            callsign,
            callpass,
            crate_version!(),
            encoded_data
        ))
        .timeout(Duration::from_secs(3))
        .send()
        .await;

    if let Ok(response) = response {
        println!("<- {}", response.status());
    } else {
        let err = response.unwrap_err();
        if err.is_timeout() {
            println!("<- TIMED OUT");
        } else {
            println!("<- ERR: {:?}", err);
        }
    }
}
