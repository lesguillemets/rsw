use std::io;
use std::io::{Write, BufWriter};
use std::thread::sleep;
use std::time::{SystemTime, Duration};

#[allow(unused_must_use)]
fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let initial = SystemTime::now();
    writer.write(b"\t");
    writer.write(b"\x1B[s");
    writer.flush();
    loop {
        sleep(Duration::from_millis(10));
        let current = SystemTime::now();
        let diff = current.duration_since(initial).unwrap();
        writer.write(b"\x1B[2K\x1B[u");
        format_duration(&diff, &mut writer);
        writer.flush();
    }
    println!("Hello, world!");
}

fn format_duration<T: Write>(d: &Duration, w: &mut BufWriter<T>) -> () {
    let ms = d.subsec_nanos() / 1_000_000;
    let whole_secs = d.as_secs();
    let (whole_mins, secs) = (whole_secs / 60, whole_secs % 60);
    let (hours, mins) = (whole_mins / 60, whole_mins % 60);
    let _ = write!(w, "{:9>}h {:2}m {:2}s {:>03}", hours, mins, secs, ms);
}
