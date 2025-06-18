use url::Url;

pub fn get_video_id(input: &str) -> Option<String> {
    if input.len() == 11 {
        return Some(input.to_string()); // assume it's just the video ID
    }

    if let Ok(url) = Url::parse(input) {
        if let Some((_, id)) = url.query_pairs().find(|(k, _)| k == "v") {
            return Some(id.to_string());
        }

        // handle youtu.be short links
        if url.domain()? == "youtu.be" {
            return Some(url.path().trim_start_matches('/').to_string());
        }
    }

    None
}

pub fn get_thumbnail_url(video_id: &str) -> String {
    format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id)
}
