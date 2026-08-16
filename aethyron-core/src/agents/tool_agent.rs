use crate::tools::filesystem::FileSystem;

pub struct ToolAgent;

impl ToolAgent {
    pub fn inspect_project(&self) {
        println!("🔍 Tool Agent inspecting project...");

        match FileSystem::list(".") {
            Ok(files) => {
                for file in files {
                    println!("📄 {}", file);
                }
            }

            Err(error) => {
                println!("❌ File inspection failed: {}", error);
            }
        }
    }
}
