use std::env;
use std::path::PathBuf;

fn main() {
    println!("[ENGINE]: re-run if proto files are changed...");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // compite proto files to rust
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile(
            &["../../proto/messages.proto", "../../proto/intent.proto"],
            &["../../proto"],
        )
        .unwrap();

}