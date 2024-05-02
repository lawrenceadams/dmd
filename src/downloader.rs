use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub api_version: String,
    pub releases: Vec<Release>,
    pub http_status: i64,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub id: String,
    pub name: String,
    pub release_date: String,
    pub archive_file_url: String,
    pub archive_file_name: String,
    pub archive_file_size_bytes: i64,
    pub archive_file_sha256: String,
    pub archive_file_last_modified_timestamp: String,
    pub checksum_file_url: String,
    pub checksum_file_name: String,
    pub checksum_file_size_bytes: i64,
    pub checksum_file_last_modified_timestamp: String,
    pub signature_file_url: String,
    pub signature_file_name: String,
    pub signature_file_size_bytes: i64,
    pub signature_file_last_modified_timestamp: String,
    pub public_key_file_url: String,
    pub public_key_file_name: String,
    pub public_key_file_size_bytes: i64,
    pub public_key_id: i64,
}

#[derive(Debug, Clone)]
struct BadFileHash;

impl std::fmt::Display for BadFileHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "calculated hash differed from the TRUD provided hash: file corrupted"
        )
    }
}

pub fn fetch_latest_release_metadata(
    release_id: u8,
    trud_key: String,
) -> Result<Root, reqwest::Error> {
    let todo = reqwest::blocking::get(format!(
        "https://isd.digital.nhs.uk/trud/api/v1/keys/{trud_key}/items/{release_id}/releases?latest"
    ))?
    .json::<Root>()?;

    Ok(todo)
}

pub fn get_and_validate_file(release: &Release) -> Result<(), Box<dyn std::error::Error>> {
    let handle = reqwest::blocking::get(&release.archive_file_url)?;
    let mut _file = std::fs::File::options()
        .create(true)
        .read(true)
        .write(true)
        .open("./dmd.zip")
        .expect("Unable to create file.");

    _file
        .write_all(&handle.bytes()?)
        .expect("Unable to write file.");

    println!(
        "  File size approx {} MB",
        _file
            .metadata()
            .expect("Unable to read file metadata")
            .len()
            / 1024
            / 1024
    );

    println!("Ensuring file integrity maintained.");

    match validate_file_hash(_file, &release.archive_file_sha256) {
        Ok(()) => return Ok(()),
        Err(BadFileHash) => panic!("Bad file hash. Cannot continue: {}", BadFileHash),
    }
}

fn validate_file_hash(
    mut file_handle: std::fs::File,
    expected_hash: &String,
) -> Result<(), BadFileHash> {
    let mut buffer = Vec::new();

    file_handle
        .seek(std::io::SeekFrom::Start(0))
        .expect("Unable to seek to start.");
    file_handle
        .read_to_end(&mut buffer)
        .expect("Unable to read to end.");

    let hash = sha256::digest(buffer).to_uppercase();

    if hash != *expected_hash {
        return Err(BadFileHash);
    }

    Ok(())
}
