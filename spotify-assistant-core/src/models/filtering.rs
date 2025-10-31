use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct ArtistLite {
    id: String,
    name: String,
    popularity: Option<i32>,
    #[serde(default)]
    genres: Vec<String>,
    #[serde(default)]
    followers: Followers,
    #[serde(default)]
    external_urls: ExternalUrls,
}
impl ArtistLite {
    pub fn name(&self) -> &str { &self.name }
    pub fn id(&self) -> &str { &self.id }
    pub fn popularity(&self) -> Option<i32> { self.popularity }
    pub fn genres(&self) -> &[String] { &self.genres }
    pub fn followers(&self) -> &i64 { &self.followers.total }
    pub fn external_urls(&self) -> &ExternalUrls { &self.external_urls }
    pub fn external_url(&self) -> &str { &self.external_urls.spotify }
    pub fn popularity_or_zero(&self) -> i32 { self.popularity.unwrap_or(0) }
    pub fn genres_or_empty(&self) -> &[String] { &self.genres }
}

#[derive(Debug, Deserialize, Clone, Default)]
struct Followers {
    #[serde(default)] total: i64
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ExternalUrls {
    #[serde(default)] spotify: String
}

pub fn load_artists_from_json(path: PathBuf) -> anyhow::Result<Vec<ArtistLite>> {
    let file = std::fs::File::open(path)?;
    let artists: Vec<ArtistLite> = serde_json::from_reader(file)?;
    Ok(artists)
}

#[allow(dead_code)]
fn filter_by_query<'a>(artists: &'a [ArtistLite], q: &str) -> Vec<&'a ArtistLite> {
    if q.is_empty() { return artists.iter().collect(); }
    let q = q.to_lowercase();
    artists.iter()
           .filter(|a| a.name.to_lowercase().contains(&q))
           .collect()
}

// example of difference: keep A \ B by id
#[allow(dead_code)]
fn difference_by_id(a: &[ArtistLite], b: &[ArtistLite]) -> Vec<ArtistLite> {
    let b_ids: HashSet<&str> = b.iter().map(|x| x.id.as_str()).collect();
    a.iter().filter(|x| !b_ids.contains(x.id.as_str())).cloned().collect()
}
