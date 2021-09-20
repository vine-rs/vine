use std::env;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "proto/";
    println!("{}", out_dir);

    // let type_dir = Path::new(&out_dir);
    // if type_dir.exists() {
    //     remove_dir_all(type_dir)?;
    // }
    // create_dir(type_dir)?;
    // let mut fmod = File::create(type_dir.join("mod.rs"))?;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

    // let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("helloworld.bin");
    // tonic_build::configure()
    //     .file_descriptor_set_path(descriptor_path)
    //     .out_dir(&out_dir)
    //     .compile(&["proto/helloworld.proto"], &["proto"])?;

    // tonic_build::configure()
    //     .out_dir(&out_dir)
    //     .compile(
    //         &["proto/apis/openapi/openapi.proto"],
    //         &["proto"],
    //     )?;

    // tonic_build::configure()
    //     .out_dir(&out_dir)
    //     .compile(
    //         &["proto/apis/registry/registry.proto"],
    //         &["proto"],
    //     )?;

    // fmod.write_all(b"pub mod errors;\n\
    // pub mod openapi;\n\
    // pub mod registry;")?;

    Ok(())
}
