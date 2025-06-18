mod youtube;
mod utils;

use std::env;

fn print_usage() {
    println!("thumbfetch - youtube thumbnail fetcher 🖼️");
    println!();
    println!("usage:");
    println!("  thumbfetch <youtube_video_url_or_id>");
    println!();
    println!("example:");
    println!("  thumbfetch https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    println!("  thumbfetch dQw4w9WgXcQ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }

    let input = &args[1];
    match youtube::get_video_id(input) {
        Some(video_id) => {
            let url = youtube::get_thumbnail_url(&video_id);
            if let Err(e) = utils::download_image(&url, &format!("{}.jpg", video_id)) {
                eprintln!("error downloading thumbnail: {}", e);
            } else {
                println!("✅ thumbnail saved as {}.jpg", video_id);
            }
        }
        None => eprintln!("❌ couldn't extract video id"),
    }
}
