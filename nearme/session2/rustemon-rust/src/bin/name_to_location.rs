use rustemon::{client::RustemonClient, model::locations::Location, locations::location};
use reqwest::Client;
use serde_json::Value;

fn get_generation_url(location: &Location) -> String {
    let game_indices = location.game_indices.as_ref().unwrap();
    let mut game_indices_clone = game_indices.clone();
    let game_index = game_indices_clone.pop().unwrap();
    let game_generation = game_index.generation.as_ref().unwrap();
    let game_generation_clone = game_generation.clone();
    let generation_url = game_generation_clone.url.unwrap();
    generation_url
}

#[tokio::main]
async fn main() {
    let rustemon_client = RustemonClient::default();
    let location =
        location::get_by_name("cerulean-city", &rustemon_client).await;
    let url = get_generation_url(&location.unwrap());
    let client = Client::new();
    let response = client.get(url).send().await;
    let res_body = response.unwrap().text().await.unwrap();
    let res_json: Value = serde_json::from_str(res_body.as_str()).unwrap();
    println!("{:?}", res_json);
}
