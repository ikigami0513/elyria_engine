use std::env;
use std::path::PathBuf;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    // Indique à Cargo de relancer ce script si un fichier dans `resources` ou `shaders` change.
    println!("cargo:rerun-if-changed=resources/*");
    println!("cargo:rerun-if-changed=shaders/*");

    // Le répertoire de destination, par ex. `target/debug` ou `target/release`.
    // On le trouve en remontant depuis le répertoire de build du crate.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.join("../../../");

    // Options de copie (ex: écraser les fichiers existants).
    let mut options = CopyOptions::new();
    options.overwrite = true;

    // Copie les dossiers.
    if std::path::Path::new("../../resources").exists() {
        copy("../../resources", &target_dir, &options).expect("Failed to copy resources folder");
    }
    if std::path::Path::new("../../shaders").exists() {
        copy("../../shaders", &target_dir, &options).expect("Failed to copy shaders folder");
    }
}