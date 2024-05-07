use std::{cmp::min, fs::File, io::Write, path::PathBuf};

use anyhow::{Context, Result};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use sp1_prover::build::dummy_proof;

/// Exports the soliditiy verifier for Groth16 proofs to the specified output directory.
///
/// WARNING: This function may take some time to complete if `SP1_DEV_WRAPPER` is enabled (which
/// is the default) as it needs to generate an end-to-end dummy proof to export the verifier.
pub fn export_solidity_groth16_verifier(output_dir: impl Into<PathBuf>) -> Result<()> {
    let output_dir: PathBuf = output_dir.into();
    let (wrap_vk, wrapped_proof) = dummy_proof();
    let artifacts_dir = sp1_prover::artifacts::get_groth16_artifacts_dir(&wrap_vk, &wrapped_proof);
    let verifier_path = artifacts_dir.join("Groth16Verifier.sol");

    if !verifier_path.exists() {
        return Err(anyhow::anyhow!(
            "verifier file not found at {:?}",
            verifier_path
        ));
    }

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory.")?;
    let output_path = output_dir.join("Groth16Verifier.sol");
    std::fs::copy(&verifier_path, output_path).context("Failed to copy verifier file.")?;

    Ok(())
}

pub async fn download_file(
    client: &Client,
    url: &str,
    file: &mut File,
) -> std::result::Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
    println!("Downloading {}", url);

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file"))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    let msg = format!("Downloaded {} to {:?}", url, file);
    pb.finish_with_message(msg);
    Ok(())
}
