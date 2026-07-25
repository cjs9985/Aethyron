use std::fs::OpenOptions;
use std::io::Write;

use anyhow::Result;


pub struct MemoryStore;


impl MemoryStore {

    pub fn save(
        content: &str,
    ) -> Result<()> {

        let mut file =
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("aethyron_memory.txt")?;


        writeln!(
            file,
            "{}",
            content
        )?;


        Ok(())
    }
}