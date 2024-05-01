use serde::{Deserialize, Serialize};

fn main() {
    let data = match fetch_latest_release_metadata(24) {
        Ok(data) => data,
        Err(_) => panic!("Something gone very wrong"),
    };

    println!("{:?}", data.releases.first().unwrap().name);
}

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

fn fetch_latest_release_metadata(release_id: u8) -> Result<Root, reqwest::Error> {
    let todo =
        reqwest::blocking::get(format!("https://isd.digital.nhs.uk/trud/api/v1/keys/f8b4b8e10055dfb6b34eb1fa7c114bd22db8201a/items/{release_id}/releases?latest"))?.json::<Root>()?;

    Ok(todo)
}
