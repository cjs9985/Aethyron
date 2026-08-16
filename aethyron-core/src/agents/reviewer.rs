use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::{ollama::OllamaClient, review_report::ReviewReport, tool_request::ToolRequest};

pub struct ReviewerAgent;

impl ReviewerAgent {
    pub async fn review(&self, task: &Task, generated_code: &str) -> ReviewReport {
        if let Err(error) = self.structural_review(generated_code) {
            return ReviewReport {
                passed: false,
                structural: false,
                security: false,
                compilation: false,
                ai_review: false,
                feedback: error,
            };
        }

        if let Err(error) = self.security_review(generated_code) {
            return ReviewReport {
                passed: false,
                structural: true,
                security: false,
                compilation: false,
                ai_review: false,
                feedback: error,
            };
        }

        if let Err(error) = self.compilation_review() {
            return ReviewReport {
                passed: false,
                structural: true,
                security: true,
                compilation: false,
                ai_review: false,
                feedback: error,
            };
        }

        let ai_feedback = match self.ai_review(task).await {
            Ok(text) => text,
            Err(error) => error,
        };

        ReviewReport {
            passed: true,
            structural: true,
            security: true,
            compilation: true,
            ai_review: true,
            feedback: ai_feedback,
        }
    }

    fn structural_review(&self, code: &str) -> Result<(), String> {
        if code.trim().is_empty() {
            return Err("Generated code is empty.".to_string());
        }

        Ok(())
    }

    fn security_review(&self, code: &str) -> Result<(), String> {
        let password_patterns = ["password: String", "password = String"];

        for pattern in password_patterns {
            if code.contains(pattern) {
                return Err(format!(
                    "Security violation: '{}' found. Passwords must never be stored as plaintext. Use bcrypt or Argon2 password hashing and store only password hashes.",
                    pattern
                ));
            }
        }

        let plaintext_comparisons = ["== password", "password ==", "!= password", "password !="];

        for pattern in plaintext_comparisons {
            if code.contains(pattern) {
                return Err(format!(
                    "Security violation: '{}' found. Do not compare plaintext passwords. Verify the supplied password against the stored password hash.",
                    pattern
                ));
            }
        }

        if code.contains(".unwrap()") {
            return Err(
                "Error-handling violation: '.unwrap()' found. Replace unwrap() with explicit error handling such as '?' or an appropriate match/Result path."
                    .to_string(),
            );
        }

        if code.contains(".expect(") {
            return Err(
                "Error-handling violation: '.expect(' found. Replace expect() with explicit error handling such as '?' or an appropriate match/Result path."
                    .to_string(),
            );
        }

        Ok(())
    }

    fn compilation_review(&self) -> Result<(), String> {
        match crate::models::compiler::Compiler::check() {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn ai_review(&self, task: &Task) -> Result<String, String> {
        let client = OllamaClient::new();

        let prompt = format!(
            "Review this engineering task:\n{}\n\n\
             Determine whether the implementation satisfies the task. \
             Focus on correctness, security, error handling, and integration. \
             Do not invent failures. If the implementation is acceptable, \
             state that clearly.",
            task.description
        );

        match client.generate(&prompt).await {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[async_trait]
impl Agent for ReviewerAgent {
    fn name(&self) -> &str {
        "Reviewer Agent"
    }

    async fn execute(&self, task: &Task) -> Option<ToolRequest> {
        println!("🔍 Review:");
        println!("{}", task.description);
        None
    }
}
