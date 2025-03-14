use crate::error::CliError;
use base64::Engine;
use clap::Parser;
use color_eyre::eyre::Result;
use dialoguer::Password;
use hc_seed_bundle::dependencies::sodoken;
use hc_seed_bundle::*;
use holochain_types::prelude::{AgentPubKey, AgentPubKeyB64};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
pub struct Unlock {
    /// Path to the seed bundle file (defaults to seed_bundle.hcsb in current directory)
    #[arg(short, long)]
    bundle: Option<PathBuf>,
}

impl Unlock {
    pub async fn execute(&self) -> Result<(), CliError> {
        // Get bundle path
        let bundle_path = match &self.bundle {
            Some(path) => path.clone(),
            None => std::env::current_dir()?.join("seed_bundle.hcsb"),
        };
        println!("bundle_path: {}", bundle_path.display());
        // Read and decode the bundle
        let encoded = std::fs::read(&bundle_path).map_err(CliError::Io)?;
        // / = String::from_utf8(encoded).unwrap();
        let bundle = base64::prelude::BASE64_URL_SAFE_NO_PAD
            .decode(&encoded)
            .map_err(|e| CliError::Config(e.to_string()))?;

        // Load the bundle
        let bundle = UnlockedSeedBundle::from_locked(&bundle)
            .await
            .map_err(|e| CliError::SeedBundle(e.to_string()))?;

        let mut unlocked = None;

        for (_, cipher) in bundle.into_iter().enumerate() {
            match cipher {
                LockedSeedCipher::PwHash(cipher) => {
                    let password = Password::new()
                        .with_prompt("Enter password")
                        .interact()
                        .map_err(|e| CliError::Config(e.to_string()))?;

                    let password = Arc::new(Mutex::new(sodoken::LockedArray::from(
                        password.as_bytes().to_vec(),
                    )));

                    match cipher.unlock(password).await {
                        Ok(u) => {
                            unlocked = Some(u);
                            break;
                        }
                        Err(_) => continue,
                    }
                }
                LockedSeedCipher::SecurityQuestions(cipher) => {
                    let question_list = cipher.get_question_list();
                    let question_list = [
                        question_list.0.clone(),
                        question_list.1.clone(),
                        question_list.2.clone(),
                    ];
                    // Unlock the question
                    println!("Question {}: {}", 1, question_list[0]);
                    let answer = Password::new()
                        .with_prompt(format!("Enter answer {}", 1))
                        .interact()
                        .map_err(|e| CliError::Config(e.to_string()))?;
                    let answer_1 = sodoken::LockedArray::from(answer.as_bytes().to_vec());

                    println!("Question {}: {}", 2, question_list[1]);
                    let answer_2 = Password::new()
                        .with_prompt(format!("Enter answer {}", 2))
                        .interact()
                        .map_err(|e| CliError::Config(e.to_string()))?;
                    let answer_2 = sodoken::LockedArray::from(answer_2.as_bytes().to_vec());

                    println!("Question {}: {}", 3, question_list[2]);
                    let answer_3 = Password::new()
                        .with_prompt(format!("Enter answer {}", 3))
                        .interact()
                        .map_err(|e| CliError::Config(e.to_string()))?;
                    let answer_3 = sodoken::LockedArray::from(answer_3.as_bytes().to_vec());

                    match cipher.unlock((answer_1, answer_2, answer_3)).await {
                        Ok(u) => {
                            unlocked = Some(u);
                            break;
                        }
                        Err(_) => continue,
                    }
                }
                _ => {
                    panic!("unsupported unknown cipher");
                }
            }
        }

        let unlocked = unlocked.ok_or_else(|| {
            CliError::Config("Failed to unlock bundle with provided credentials".into())
        })?;

        // Get the seed and convert to keys
        let pub_key = unlocked.get_sign_pub_key();
        // Convert public key to AgentPubKeyB64
        let public_key = AgentPubKey::from_raw_32(pub_key.as_slice().to_vec());
        let public_key_b64 = AgentPubKeyB64::from(public_key);

        println!("\nPublic Key (raw): {}", hex::encode(pub_key.as_slice()));
        println!("Holochain Public Key (AgentPubKeyB64): {}", public_key_b64);

        Ok(())
    }
}
