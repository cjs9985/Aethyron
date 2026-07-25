use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

use anyhow::Result;
use serde_json;

use crate::models::mission_result::MissionResult;


pub struct MemoryStore;


impl MemoryStore {

    const FILE: &'static str = "aethyron_memory.json";


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


    pub fn save_result(
        result: &MissionResult,
    ) -> Result<()> {

        let json =
            serde_json::to_string(result)?;


        Self::save(&json)
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