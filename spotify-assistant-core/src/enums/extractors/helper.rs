use rspotify::model::{AlbumType, DatePrecision, Restriction, RestrictionReason};

pub trait ExtractorHelper {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn restrict_reason_as_string(wrapped_restriction: &Option<Restriction>) -> String {
        match wrapped_restriction {
            Some(restriction) => match restriction.reason {
                RestrictionReason::Market => String::from("market"),
                RestrictionReason::Product => String::from("product"),
                RestrictionReason::Explicit => String::from("explicit"),
            },
            None => String::from("none"),
        }
    }
    fn date_precision_as_string(date_precision: &DatePrecision) -> String {
        match date_precision {
            DatePrecision::Year => String::from("year"),
            DatePrecision::Month => String::from("month"),
            DatePrecision::Day => String::from("day"),
        }
    }
    fn album_type_as_string(album_type: &AlbumType) -> String {
        match album_type {
            AlbumType::Album => String::from("album"),
            AlbumType::Single => String::from("single"),
            AlbumType::Compilation => String::from("compilation"),
            AlbumType::AppearsOn => String::from("appears_on"),
        }
    }
}
