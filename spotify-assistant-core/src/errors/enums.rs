use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnumError {
    #[error("Cannot {action} because this action is not available for the {variant_label} variant")]
    /// Cannot {action} because this action is not available for the {variant_label} variant
    NotAvailableForVariant {
        /// Example: Cannot {action} because ...
        action: &'static str,
        /// Example: Cannot ... because this is not available for the {variant_label}
        variant_label: &'static str,
    },
    #[error("Could not extract {output_variant} from {src_variant}: {err}")]
    ExtractorInstantiationError {
        /// Example: Couldn't extract {output_variant} ...
        output_variant: &'static str,
        /// Example: Couldn't ... from {src_variant}
        src_variant: &'static str,
        err: anyhow::Error,
    },
    #[error("Error raised from returning a None value")]
    NoneReturned,
    #[error("Unknown error occurred in collection processing")]
    Unknown,
}
