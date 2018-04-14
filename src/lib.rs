//! This is a small wrapper for the [`upspin`](https://upspin.io) tool.
#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};

use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use std::fs::File;

/// The path to a file in upspin
pub struct UpspinPath {
    owner: String,
    path: String,
}

impl UpspinPath {
    /// Returns the owner portion of the `UpspinPath`.
    ///
    /// For "augie@upspin.io/Images/Augie/small.jpg" this would be "augie".
    pub fn owner(&self) -> &String {
        &self.owner
    }

    /// Returns the path portion of the `UpspinPath`.
    ///
    /// For "augie@upspin.io/Images/Augie/small.jpg" this would be "Images/Augie/small.jpg".
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Returns the file name of the `UpspinPath`.
    ///
    /// For "augie@upspin.io/Images/Augie/small.jpg" this would be "small.jpg".
    pub fn file_name(&self) -> &str {
        &self.path.split("/").last().unwrap()
    }

    /// Returns the full upspin path of the `UpspinPath`.
    ///
    /// For "augie@upspin.io/Images/Augie/small.jpg" this would be "augie@upspin.io/Images/Augie/small.jpg".
    pub fn full_path(&self) -> String {
        format!("{}/{}", &self.owner, &self.path)
    }

    /// Checks if the file is publicly available.
    pub fn is_public(&self) -> bool {
        match Command::new("upspin")
            .args(&["info", &self.full_path()])
            .output()
        {
            Ok(output) => {
                output.status.success()
                    && std::str::from_utf8(&output.stdout)
                        .context("Could not parse output")
                        .unwrap()
                        .lines()
                        .any(|line| line.contains("can read") && line.contains("All"))
            }
            Err(_) => false,
        }
    }

    /// Downloads the file from upspin to the provided output path.
    ///
    /// Only downloads publicly available files.
    pub fn get(&self, output_path: &Path) -> Result<(), Error> {
        if !self.is_public() {
            return Err(format_err!("Path {} is not public", self.full_path()));
        }

        let output_file = File::create(output_path)
            .context("Could not create output file")
            .unwrap();

        let output = Command::new("upspin")
            .args(&["get", &self.full_path()])
            .stdout(Stdio::from(output_file))
            .output();

        match output {
            Ok(output) => if output.status.success() {
                Ok(())
            } else {
                Err(format_err!("Could not download"))
            },
            Err(e) => Err(format_err!("Could not download: {}", e)),
        }
    }
}

impl FromStr for UpspinPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_s = s.split("/");
        let owner = splitted_s.next().unwrap().to_string();
        let path = splitted_s.collect::<Vec<_>>().join("/");
        Ok(UpspinPath { owner, path })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn asdf() {
        let path = "augie@upspin.io/Images/Augie/small.jpg";

        let upspin_path: UpspinPath = path.parse().unwrap();

        assert_eq!(&"augie@upspin.io".to_string(), upspin_path.owner());
        assert_eq!(&"Images/Augie/small.jpg".to_string(), upspin_path.path());
        assert_eq!(&"small.jpg".to_string(), upspin_path.file_name());
        assert_eq!(path, upspin_path.full_path());
    }
}
