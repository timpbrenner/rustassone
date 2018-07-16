use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use serde_json;

#[derive(Serialize)]
pub struct JsTile {
    pub id: i32,
    pub playerId: i32,
    pub cities: Vec<i32>,
    pub roads: Vec<i32>,
    pub rowOffset: i32,
    pub columnOffset: i32,
    pub meeple: Option<JsMeeple>,
}

impl Responder for JsTile {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Serialize)]
pub struct JsMeeple {
    pub playerId: i32,
    pub tileId: i32,
    pub side: i32,
}

#[derive(Serialize)]
pub struct JsPlayer {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct JsGame {
    pub id: i32,
    pub grid: Vec<Vec<JsTile>>,
    pub players: Vec<JsPlayer>,
    pub currentPlayerId: i32,
    pub currentState: String,
    pub currentTile: Option<JsTile>,
}

impl Responder for JsGame {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}


