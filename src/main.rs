use axum::{
    routing::post,
    extract::Multipart,
    response::IntoResponse,
    Router,
};
use std::{net::SocketAddr, io::Write};
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::formats::FormatOptions;
use tempfile::NamedTempFile;
use hyper::Server;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/analyze", post(analyze_audio));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Serveur lancé sur http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn analyze_audio(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("file").to_string(); // 🔑 Clone ici
        let data = field.bytes().await.unwrap();               // Puis consomme field

        let mut tempfile = NamedTempFile::new().unwrap();
        tempfile.write_all(&data).unwrap();
        let path = tempfile.path().to_path_buf();

        let file = std::fs::File::open(&path).unwrap();
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let hint = Hint::new();
        let probed = get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
            .unwrap();
        let format = probed.format;
        let track_count = format.tracks().len();

        let mut result = format!("Fichier reçu: {}\nNombre de pistes: {}\n", name, track_count);
        for (i, track) in format.tracks().iter().enumerate() {
            result.push_str(&format!("Piste {} : {:?}\n", i + 1, track.codec_params));
        }

        return result.into_response();
    }

    "Aucun fichier trouvé.".into_response()
}
