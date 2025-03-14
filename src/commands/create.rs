use crate::error::CliError;
use base64::Engine;
use clap::Parser;
use color_eyre::eyre::Result;
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

        println!("Select cipher type:");
        println!("1. Password");
        println!("2. Security Questions");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let seed = hc_seed_bundle::UnlockedSeedBundle::new_random()
            .await
            .map_err(|e| CliError::SeedBundle(e.to_string()))?;

        let mut cipher = seed.lock();

        match input.trim() {
            "1" => {
                println!("Enter password:");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password)?;
                let password = Arc::new(Mutex::new(sodoken::LockedArray::from(
                    password.trim().as_bytes().to_vec(),
                )));

                cipher =
                    PwHashLimits::Minimum.with_exec(move || cipher.add_pwhash_cipher(password));
            }
            "2" => {
                println!("Enter question 1:");
                let mut q1 = String::new();
                std::io::stdin().read_line(&mut q1)?;

                println!("Enter answer 1:");
                let mut a1 = String::new();
                std::io::stdin().read_line(&mut a1)?;

                println!("Enter question 2:");
                let mut q2 = String::new();
                std::io::stdin().read_line(&mut q2)?;

                println!("Enter answer 2:");
                let mut a2 = String::new();
                std::io::stdin().read_line(&mut a2)?;

                println!("Enter question 3:");
                let mut q3 = String::new();
                std::io::stdin().read_line(&mut q3)?;

                println!("Enter answer 3:");
                let mut a3 = String::new();
                std::io::stdin().read_line(&mut a3)?;

                let questions = (
                    q1.trim().to_string(),
                    q2.trim().to_string(),
                    q3.trim().to_string(),
                );

                let answers = (
                    sodoken::LockedArray::from(a1.trim().as_bytes().to_vec()),
                    sodoken::LockedArray::from(a2.trim().as_bytes().to_vec()),
                    sodoken::LockedArray::from(a3.trim().as_bytes().to_vec()),
                );

                cipher = cipher.add_security_question_cipher(questions, answers);
            }
            _ => return Err(CliError::Config("Invalid cipher type".into())),
        }

        let bundle = cipher
            .lock()
            .await
            .map_err(|e| CliError::SeedBundle(e.to_string()))?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(&bundle);
        std::fs::write(&output_path, encoded).map_err(|e| CliError::Io(e))?;

        Ok(())
    }
}
