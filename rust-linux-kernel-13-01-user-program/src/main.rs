use std::{fs::File, io::Read, path::Path};

fn main() -> std::io::Result<()> {
    let mut rng = File::open(Path::new("/dev/rust-random-file")).expect("Can't find /dev/rust-random-file");
    let mut buffer = [0u8; 16];

    rng.read_exact(&mut buffer).expect("Can't read exact buffer");
    println!("랜덤 숫자: {buffer:?}");

    Ok(())
}
