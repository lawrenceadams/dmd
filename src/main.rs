use std::env;
mod downloader;

fn main() {
    println!("Start.");

    let trud_key = env::var("TRUD_KEY").expect("No TRUD key in environment variables.");

    let data = match downloader::fetch_latest_release_metadata(24, trud_key.as_str()) {
        Ok(data) => {
            println!("Got manifest ok.");
            data
        }
        Err(_) => panic!("Failed to get manifest file for release_id."),
    };

    downloader::get_and_validate_file(&data.releases[0]).expect("Unknown error occurred.");

    println!("File fetched successfully!");
}
