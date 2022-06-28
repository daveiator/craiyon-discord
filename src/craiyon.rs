//shoutout to https://github.com/JelNiSlaw who created this code for his telegram bot

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Payload {
    prompt: String,
}

#[derive(Deserialize)]
struct Response {
    images: Vec<String>,
}

pub async fn generate(prompt: String) -> reqwest::Result<Vec<Vec<u8>>> {
    let client = reqwest::Client::new();
    let body = Payload {
        prompt,
    };
    info!("Sending request to craiyon.com");
    let response = match client
        .post("https://backend.craiyon.com/generate")
        .json(&body)
        .send()
        .await?
        .error_for_status()
    {
        Ok(response) => {
            info!("Received images from craiyon.com");
            response.json::<Response>().await?
        },
        Err(err) => {
            error!("Couldn't get images from craiyon.com: {}", err);
            return Err(err)
        },
    };
    let images = response
        .images
        .into_iter()
        .map(|data| base64::decode(data.replace('\n', "")).unwrap())
        .collect();

    Ok(images)
}