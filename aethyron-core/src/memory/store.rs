use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

use anyhow::Result;


pub struct MemoryStore;


impl MemoryStore {

    const FILE: &'static str = "aethyron_memory.txt";


    pub fn save(
        content: &str,
    ) -> Result<()> {

        let mut file =
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(Self::FILE)?;


        writeln!(
            file,
            "{}",
            content
        )?;


        Ok(())
    }


    pub fn load() -> Result<String> {

        match read_to_string(Self::FILE) {

            Ok(memory) => Ok(memory),

            Err(error) => {

                if error.kind()
                    == std::io::ErrorKind::NotFound
                {
                    Ok(String::new())
                }

                else {
                    Err(error.into())
                }
            }
        }
    }
}