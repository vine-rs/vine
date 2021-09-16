use std::path::Path;
use std::fs::{remove_dir_all, create_dir, File};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let out_dir = "src/types";
    // println!("{}", out_dir);

    // let type_dir = Path::new(&out_dir);
    // if type_dir.exists() {
    //     remove_dir_all(type_dir)?;
    // }
    // create_dir(type_dir)?;
    // let mut fmod = File::create(type_dir.join("mod.rs"))?;


    // tonic_build::configure()
    //     .out_dir(&out_dir)
    //     .compile(
    //         &["proto/apis/errors/errors.proto"],
    //         &["proto"],
    //     )?;

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