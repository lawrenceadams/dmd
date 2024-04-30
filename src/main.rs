use error_chain::error_chain;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://isd.digital.nhs.uk/trud/api/v1/keys/f8b4b8e10055dfb6b34eb1fa7c114bd22db8201a/items/24/releases?latest").await?.text().await?;

    println!("Body:\n{}", &res);

    let parsed: serde_json::Value = serde_json::from_str(&res).expect("Bad formatting");

    let fileurl = parsed
        .get("releases")
        .and_then(|releases_list| releases_list.get(0))
        .and_then(|release| release.get("archiveFileUrl"))
        .expect("Bad API Key or URL")
        .as_str()
        .expect("Unable to parse 'archiveFileUrl' to string.");

    println!("{}", &fileurl);

    let path = Path::new("./latest.zip");

    let mut file = match File::create(path) {
        Err(why) => panic!("Could not create file: {why}"),
        Ok(file) => file,
    };

    let res = reqwest::get(fileurl).await?.bytes().await?;
    file.write_all(&res).unwrap();

    Ok(())
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    id: String,
    name: String,
    releaseDate: String,
    archiveFileUrl: String,
    archiveFileName: String,
    archiveFileSizeBytes: u32,
    archiveFileSha256: String,
    archiveFileLastModifiedTimestamp: String,
    checksumFileUrl: String,
    checksumFileName: String,
    checksumFileSizeBytes: u32,
    checksumFileLastModifiedTimestamp: String,
    signatureFileUrl: String,
    signatureFileName: String,
    signatureFileSizeBytes: u32,
    signatureFileLastModifiedTimestamp: String,
    publicKeyFileUrl: String,
    publicKeyFileName: String,
    publicKeyFileSizeBytes: u32,
    publicKeyId: u32,
}

async fn get_trud_asset(
    api_key: String,
    release_id: u8,
) -> Result<serde_json::Value, reqwest::Error> {
    let request_url = _trud_asset_latest_url(api_key, release_id).await;
    let res = reqwest::get(request_url).await?.json::<Payload>().await?

    let parsed: serde_json::Value = serde_json::from_str(&res).expect("Bad formatting");

    Ok(parsed)
}

async fn _trud_asset_latest_url(api_key: String, release_id: u8) -> reqwest::Url {
    let api_url = format!(
        "https://isd.digital.nhs.uk/trud/api/v1/keys/{api_key}/items/{release_id}/releases?latest"
    );
    reqwest::Url::from_str(&api_url).expect("Unable to build URL from provided inputs")
}
