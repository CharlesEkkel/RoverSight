use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub struct Rover {
    pub id: i32,
    pub name: String,
    pub landing_date: String,
    pub launch_date: String,
    pub status: String,
}

#[derive(Deserialize, PartialEq)]
pub struct Camera {
    pub id: i32,
    pub name: i32,
    pub full_name: String,
    pub rover_id: i32,
}

#[derive(Deserialize, PartialEq)]
pub struct Photo {
    pub id: i32,
    pub sol: i32,
    pub img_src: String,
    pub earth_date: String,
    pub rover: Rover,
}

#[derive(Deserialize, PartialEq)]
struct Photos {
    photos: Vec<Photo>
}

pub async fn get_photos(rover: RoverName) -> Result<Vec<Photo>, reqwest::Error> {
    let client = Client::new();
    let response = client.get(
        format!("https://api.nasa.gov/mars-photos/api/v1/rovers/{rover_str}/photos?sol=1000&camera=fhaz&api_key=DEMO_KEY"
                , rover_str = rover.as_str())
    ).send().await?;

    response.json::<Photos>().await.map(|photos_obj| photos_obj.photos)
}

#[derive(PartialEq, Clone, Copy)]
pub enum RoverName {
    Curiosity,
    Opportunity,
    Spirit
}

impl RoverName {
    fn as_str(&self) -> String {
        String::from(match &self {
            RoverName::Curiosity => "curiosity",
            RoverName::Opportunity => "opportunity",
            RoverName::Spirit => "spirit"
        })
    }
}
