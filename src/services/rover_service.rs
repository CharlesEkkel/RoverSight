use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Rover {
    id: i32,
    name: String,
    landing_date: String,
    launch_date: String,
    status: String,
}

#[derive(Deserialize, Clone)]
struct Camera {
    id: i32,
    name: i32,
    full_name: String,
    rover_id: i32,
}

#[derive(Deserialize, Clone)]
struct Photo {
    id: i32,
    sol: i32,
    img_src: String,
    earth_date: String,
    rover: Rover,
}

#[derive(Deserialize, Clone)]
struct Photos {
    photos: Vec<Photo>,
}

pub async fn get_photos() -> Result<Photos, reqwest::Error> {
    let client = Client::new();
    let response = client.get(
        "https://api.nasa.gov/mars-photos/api/v1/rovers/curiosity/photos?sol=1000&api_key=DEMO_KEY"
    ).send().await?;

    response.json::<Photos>().await
}
