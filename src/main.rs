use symphonia::default::*;
use symphonia::core::io::MediaSourceStream;
use std::fs::File;

fn main() {
    let path = "example.mp3";

    let file = File::open(path).expect("Échec ouverture fichier");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &Default::default(),
        &Default::default(),
    ).expect("Échec détection format");

    let format = probed.format;

    println!("Format détecté avec succès !");
    println!("Nombre de pistes : {}", format.tracks().len());

    for (i, track) in format.tracks().iter().enumerate() {
        println!("Piste {} : {:?}", i + 1, track.codec_params);
    }
}
