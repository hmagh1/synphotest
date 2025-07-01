use axum::{routing::get, response::Json, Router};
use std::{fs::File, net::SocketAddr};
use symphonia::default::*;
use symphonia::core::io::MediaSourceStream;
use serde_json::json;

async fn analyze() -> Json<serde_json::Value> {
    let file = match File::open("example.mp3") {
        Ok(f) => f,
        Err(_) => return Json(json!({ "error": "Fichier non trouvé" })),
    };

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = match get_probe().format(
        &Default::default(),
        mss,
        &Default::default(),
        &Default::default(),
    ) {
        Ok(p) => p,
        Err(_) => return Json(json!({ "error": "Format non reconnu" })),
    };

    let format = probed.format;

    Json(json!({
        "format": format.format_name(), // ✅ correction ici
        "nb_tracks": format.tracks().len()
    }))
}


#[tokio::main]
async fn main() {
    let app = Router::new().route("/analyze", get(analyze));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("Serveur lancé sur http://0.0.0.0:8080");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}

