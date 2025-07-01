use symphonia::default::*;
use symphonia::core::io::MediaSourceStream;
use std::fs::File;

fn main() {
    let path = "example.mp3"; // mets ton fichier ici

    let file = File::open(path).expect("Échec ouverture fichier");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &Default::default(),
        &Default::default(),
    ).expect("Échec détection format");

    let mut format = probed.format;
    println!("Format détecté : {:?}", format);
}


