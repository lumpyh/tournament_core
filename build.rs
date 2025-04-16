use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tournament_descriptor.bin"))
        .compile_protos(
            &[
                "tournament_protos/tournament.proto",
                "tournament_protos/messages.proto",
                "tournament_protos/fencer_messages.proto",
            ],
            &["tournament_protos"],
        )?;
    Ok(())
}
