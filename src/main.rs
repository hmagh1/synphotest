use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{io::Write, net::SocketAddr};
use symphonia::core::{
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};
use symphonia::default::get_probe;
use tempfile::NamedTempFile;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/analyze", post(analyze_audio));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Serveur lancé sur http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn analyze_audio(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();

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
