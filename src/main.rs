use axum::{routing::get, Router, Json};
use std::{net::SocketAddr, fs::File};
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::formats::FormatOptions;
use serde::Serialize;

#[derive(Serialize)]
struct AudioInfo {
    format: String,
    nb_tracks: usize,
    first_track_codec: String,
}

async fn analyze_audio() -> Json<AudioInfo> {
    let file = File::open("example.mp3").expect("cannot open file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &FormatOptions::default(),
        mss,
        &MetadataOptions::default(),
        &Default::default()
    ).expect("format error");

    let format = probed.format;

    let nb_tracks = format.tracks().len();
    let first_codec = format.tracks()
        .get(0)
        .map(|t| format!("{:?}", t.codec_params.codec))
        .unwrap_or("unknown".to_string());

    Json(AudioInfo {
        format: format!("Symphonia"), // ou une info statique selon ton besoin
        nb_tracks,
        first_track_codec: first_codec,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/analyze", get(analyze_audio));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Serveur sur http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
