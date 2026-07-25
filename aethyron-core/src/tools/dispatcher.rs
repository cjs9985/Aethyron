use crate::models::tool_request::ToolRequest;
use crate::tools::filesystem::FileSystem;


pub struct ToolDispatcher;


impl ToolDispatcher {

    pub fn execute(
        request: ToolRequest,
    ) -> String {

        match request {

            ToolRequest::InspectProject => {

                let result =
                    FileSystem::inspect_project_result();

                result.output
            }

        }
    }
}