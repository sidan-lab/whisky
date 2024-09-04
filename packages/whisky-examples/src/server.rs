use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use whisky::{csl::JsError, model::UTxO};
use whisky_examples::tx;

fn response(result: Result<String, JsError>) -> HttpResponse {
    match result {
        Ok(tx_hex) => HttpResponse::Ok().json(TxResponse { tx_hex }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[derive(Serialize)]
struct TxResponse {
    tx_hex: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct TxRequest {
    pub recipient_address: String,
    pub my_address: String,
    pub inputs: Vec<UTxO>,
}

async fn send_lovelace(req: web::Json<TxRequest>) -> impl Responder {
    let res = tx::send_lovelace(&req.recipient_address, &req.my_address, &req.inputs);
    response(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/send_lovelace", web::post().to(send_lovelace)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
