use dialoguer::{FuzzySelect, MultiSelect, theme::ColorfulTheme};
use spotify_assistant_core::models::filtering::ArtistLite;

#[allow(dead_code)]
pub fn pick_single_artist(candidates: &[ArtistLite]) -> Option<&ArtistLite> {
    let items: Vec<String> = candidates
        .iter()
        .map(|a| {
            format!(
                "{}  (pop: {}, followers: {})",
                a.name(),
                a.popularity().unwrap_or(0),
                a.followers()
            )
        })
        .collect();

    let idx = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Search artist")
        .items(&items)
        .default(0)
        .interact_opt()
        .ok()??;

    candidates.get(idx)
}

pub fn pick_multiple_artists(candidates: &[ArtistLite]) -> Vec<&ArtistLite> {
    let items: Vec<String> = candidates
        .iter()
        .map(|a| {
            format!(
                "{} (pop: {}, followers: {})",
                a.name(),
                a.popularity().unwrap_or(0),
                a.followers()
            )
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select one or more artists (space to toggle, enter to confirm)")
        .items(&items)
        .interact()
        .unwrap_or_default();

    selections
        .into_iter()
        .filter_map(|idx| candidates.get(idx))
        .collect()
}
