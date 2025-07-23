#[macro_export]
macro_rules! collect_track_field {
    ($tracks:expr, $field_path:expr, $default:expr) => {
        Some($tracks.iter().map(|track| {
            match $field_path(track).clone() {
                Some(field) => field,
                None => $default,
            }
        }).collect())
    };
    // Variant without Option handling for direct field access
    ($tracks:expr, $field_path:expr) => {
        Some($tracks.iter().map(|track| $field_path(track)).collect())
    };
}
