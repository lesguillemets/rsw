use std::io;
use std::io::{Write, BufWriter};
use std::thread::sleep;
use std::time;
use std::time::SystemTime;

fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let initial = SystemTime::now();
    writer.write(b"\t");
    writer.write(b"\x1B[s");
    writer.flush();
    loop {
        sleep(time::Duration::from_millis(10));
        let current = SystemTime::now();
        let diff = current.duration_since(initial).unwrap();
        if diff.as_secs() > 1 {
            writer.write(b"\x1B[2K\x1B[u");
            writer
                .write(format!("{} {}", diff.as_secs(), diff.subsec_nanos() / 1000000).as_bytes());
            writer.flush();
        }
    }
    println!("Hello, world!");
}
