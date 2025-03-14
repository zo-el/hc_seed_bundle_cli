use crate::error::CliError;
use base64::Engine;
use clap::Parser;
use color_eyre::eyre::Result;
use dialoguer::{Input, Password, Select};
use hc_seed_bundle::{dependencies::sodoken, *};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const BUNDLE_FILENAME: &str = "seed_bundle.hcsb";

#[derive(Parser, Debug)]
pub struct Create {
    /// Directory where the seed_bundle.hcsb will be saved (defaults to current directory)
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

impl Create {
    pub async fn execute(&self) -> Result<(), CliError> {
        // Get output directory (current dir if not specified) and append filename
        let output_path = match &self.directory {
            Some(dir) => dir.join(BUNDLE_FILENAME),
            None => std::env::current_dir()?.join(BUNDLE_FILENAME),
        };

        let options = vec!["Password", "Security Questions"];
        let selection = Select::new()
            .with_prompt("Select cipher type")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| CliError::Config(e.to_string()))?;

        let seed = hc_seed_bundle::UnlockedSeedBundle::new_random()
            .await
            .map_err(|e| CliError::SeedBundle(e.to_string()))?;

        let mut cipher = seed.lock();

        match selection {
            0 => {
                let password = Password::new()
                    .with_prompt("Enter password")
                    .interact()
                    .map_err(|e| CliError::Config(e.to_string()))?;

                let password = Arc::new(Mutex::new(sodoken::LockedArray::from(
                    password.as_bytes().to_vec(),
                )));

                cipher =
                    PwHashLimits::Minimum.with_exec(move || cipher.add_pwhash_cipher(password));
            }
            1 => {
                let q1: String = Input::new()
                    .with_prompt("Enter question 1")
                    .interact_text()
                    .map_err(|e| CliError::Config(e.to_string()))?;
                let a1: String = Password::new()
                    .with_prompt("Enter answer 1")
                    .interact()
                    .map_err(|e| CliError::Config(e.to_string()))?;

                let q2: String = Input::new()
                    .with_prompt("Enter question 2")
                    .interact_text()
                    .map_err(|e| CliError::Config(e.to_string()))?;
                let a2: String = Password::new()
                    .with_prompt("Enter answer 2")
                    .interact()
                    .map_err(|e| CliError::Config(e.to_string()))?;

                let q3: String = Input::new()
                    .with_prompt("Enter question 3")
                    .interact_text()
                    .map_err(|e| CliError::Config(e.to_string()))?;
                let a3: String = Password::new()
                    .with_prompt("Enter answer 3")
                    .interact()
                    .map_err(|e| CliError::Config(e.to_string()))?;

                let questions = (q1, q2, q3);
                let answers = (
                    sodoken::LockedArray::from(a1.as_bytes().to_vec()),
                    sodoken::LockedArray::from(a2.as_bytes().to_vec()),
                    sodoken::LockedArray::from(a3.as_bytes().to_vec()),
                );

                cipher = cipher.add_security_question_cipher(questions, answers);
            }
            _ => unreachable!(),
        }

        let bundle = cipher
            .lock()
            .await
            .map_err(|e| CliError::SeedBundle(e.to_string()))?;

        let encoded = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(bundle);
        std::fs::write(&output_path, encoded).map_err(|e| CliError::Io(e))?;

        Ok(())
    }
}
