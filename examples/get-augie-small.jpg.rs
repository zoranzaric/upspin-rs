extern crate upspin;

use std::path::Path;

/// Downloads a small Augie JPEG to the current directory.
pub fn main() {
    let upspin_path: upspin::UpspinPath = "augie@upspin.io/Images/Augie/small.jpg".parse().unwrap();

    let output_path = Path::new(upspin_path.file_name());

    match upspin_path.get(&output_path) {
        Ok(_) => println!("Downloaded file {}", upspin_path.file_name()),
        Err(e) => eprintln!("{}", e),
    }
}
