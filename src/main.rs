mod downloader;

fn main() {
    println!("Start.");

    let data = match downloader::fetch_latest_release_metadata(24) {
        Ok(data) => data,
        Err(_) => panic!("Something gone very wrong"),
    };

    match downloader::get_and_validate_file(&data.releases[0]) {
        Ok(()) => println!("Completed okay."),
        Err(_) => panic!("Fucked it"),
    };
}
