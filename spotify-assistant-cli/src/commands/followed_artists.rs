use anyhow::Result;
use spotify_assistant_core::models::filtering::{load_artists_from_json, ArtistLite};
use std::path::PathBuf;

pub fn cmd_find_artists(json_path: PathBuf) -> Result<Vec<ArtistLite>> {
    let artists: Vec<ArtistLite> = load_artists_from_json(json_path)?;

    let picked = crate::ui::fuzzy::pick_multiple_artists(&artists);
    let mut new = Vec::new();
    if picked.is_empty() {
        println!("No artists selected");
    } else {
        for a in picked.clone() {
            println!("You picked: {} ({})", a.name(), a.id());
            new.push(a.clone())
        }
    }
    Ok(new)
}
