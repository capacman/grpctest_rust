use std::path::Path;

fn main() {
    let tlcgen2 = Path::new("schemas/test.proto");
    let dir = tlcgen2.parent().unwrap();
    tonic_build::configure()
        .build_server(true)
        .out_dir("schemas")
        .compile(&[tlcgen2], &[dir])
        .unwrap();
}
