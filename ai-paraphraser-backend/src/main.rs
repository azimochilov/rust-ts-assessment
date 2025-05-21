use actix_cors::Cors;
use actix_web::{web, HttpResponse, Responder};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use std::env;

#[derive(Deserialize)]
struct ParaphraseRequest {
    text: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

async fn paraphrase(req: web::Json<ParaphraseRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");

    let prompt = format!("Paraphrase this text: {}", req.text);

    let openai_req = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&openai_req)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "No response body.".to_string());

            if status.is_success() {
                match serde_json::from_str::<OpenAIResponse>(&text) {
                    Ok(data) => HttpResponse::Ok().body(data.choices[0].message.content.clone()),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to parse response."),
                }
            } else {
                HttpResponse::InternalServerError().body("OpenAI returned an error.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Request to OpenAI failed."),
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static> {
    dotenv().ok();

    let app = move |cfg: &mut web::ServiceConfig| {
        let cors = Cors::permissive(); // Allows all origins. For production, configure appropriately.

        cfg.service(
            web::resource("/paraphrase")
                .wrap(cors)
                .route(web::post().to(paraphrase)),
        );
    };

    Ok(app.into())
}
