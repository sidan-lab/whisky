use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use whisky::{
    csl::JsError,
    model::{ProvidedScriptSource, UTxO},
};
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
#[serde(rename_all = "camelCase")]
struct TxResponse {
    tx_hex: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendLovelaceRequest {
    pub recipient_address: String,
    pub my_address: String,
    pub inputs: Vec<UTxO>,
}

async fn send_lovelace(req: web::Json<SendLovelaceRequest>) -> impl Responder {
    let res = tx::send_lovelace(&req.recipient_address, &req.my_address, &req.inputs);
    response(res)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LockFundRequest {
    pub script_address: String,
    pub datum: String,
    pub my_address: String,
    pub inputs: Vec<UTxO>,
}

async fn lock_fund(req: web::Json<LockFundRequest>) -> impl Responder {
    let res = tx::lock_fund(
        &req.script_address,
        &req.datum,
        &req.my_address,
        &req.inputs,
    );
    response(res)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnlockFundRequest {
    pub script_utxo: UTxO,
    pub redeemer: String,
    pub script: ProvidedScriptSource,
    pub my_address: String,
    pub inputs: Vec<UTxO>,
    pub collateral: UTxO,
}

async fn unlock_fund(req: web::Json<UnlockFundRequest>) -> impl Responder {
    let res = tx::unlock_fund(
        &req.script_utxo,
        &req.redeemer,
        &req.script,
        &req.my_address,
        &req.inputs,
        &req.collateral,
    )
    .await;
    response(res)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MintTokensRequest {
    pub to_mint_asset: whisky::model::Asset,
    pub redeemer: String,
    pub script: ProvidedScriptSource,
    pub my_address: String,
    pub inputs: Vec<UTxO>,
    pub collateral: UTxO,
}

async fn mint_tokens(req: web::Json<MintTokensRequest>) -> impl Responder {
    let res = tx::mint_tokens(
        &req.to_mint_asset,
        &req.redeemer,
        &req.script,
        &req.my_address,
        &req.inputs,
        &req.collateral,
    )
    .await;
    response(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/send_lovelace", web::post().to(send_lovelace))
            .route("/lock_fund", web::post().to(lock_fund))
            .route("/unlock_fund", web::post().to(unlock_fund))
            .route("/mint_tokens", web::post().to(mint_tokens))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
