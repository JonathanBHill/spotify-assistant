use ansi_term::Color;
use tracing::Level;
use tracing_subscriber::fmt::time::{ChronoLocal, FormatTime};
use tracing_subscriber::fmt::{
    format::Writer, FmtContext, FormatEvent, FormatFields, FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

/// A custom formatter struct used for formatting purposes.
///
/// This struct is typically employed in scenarios where custom formatting logic
/// for specific data types or outputs is necessary. The use of the `#[cfg(not(tarpaulin_include))]`
/// attribute ensures that this struct is excluded from coverage analysis by the `tarpaulin` code
/// coverage tool, thereby preventing its inclusion in test coverage reports.
///
/// # Example
/// ```ignore
/// let formatter = CustomFormatter;
/// // Use `formatter` to apply custom formatting logic
/// ```
///
/// # Attributes
/// None
///
/// # Notes
/// This struct does not contain any fields or methods by default and is expected
/// to be extended or used in conjunction with other logic to serve as a formatting utility.
///
/// # Feature Flags
/// - This struct is conditionally excluded from test coverage reporting when
///   compiled with the `tarpaulin` tool due to the inclusion of the `cfg(not(tarpaulin_include))`
///   conditional compilation attribute.
#[cfg(not(tarpaulin_include))]
pub struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
        N: for<'a> FormatFields<'a> + 'static,
{
    /// Formats a `tracing::Event` for logging output with custom formatting, including
    /// time, log level, module path, and optional span field data styling.
    ///
    /// # Parameters
    /// - `ctx`: A reference to the `FmtContext`, providing the context for formatting
    ///   the event and managing field formatting and spans.
    /// - `writer`: A `Writer` object to which the formatted log is written.
    /// - `event`: The `tracing::Event` to be formatted and logged.
    ///
    /// # Returns
    /// - A `std::fmt::Result`, indicating success or failure of the formatting operation.
    ///
    /// # Behavior
    /// 1. Retrieves metadata about the event, including log level, line number, and target.
    /// 2. Formats the current timestamp using a specific time format (`"%H:%M:%S%.3f"`).
    /// 3. Styles various components of the log entry:
    ///    - The target (module path) is formatted in purple.
    ///    - The line number is formatted in bold purple.
    ///    - Log levels are color-coded:
    ///       - `INFO`: Green
    ///       - `WARN`: Yellow
    ///       - `ERROR`: Red
    ///       - `DEBUG`: Blue
    ///       - `TRACE`: White
    /// 4. If the event originates within a scope, iterates through the span hierarchy
    ///    (from root to the event) and formats the spans:
    ///    - Span names are colored green and bolded.
    ///    - Arrows between spans are colored orange and bolded (with a blinking effect).
    ///    - Span fields, if present, are italicized and cyan-colored.
    /// 5. Formats the event's fields and message, applying custom styling, such as italicization.
    ///
    /// # Usage
    /// This function is invoked during custom formatting of events for a logger implementation
    /// that uses the `tracing` crate. It is not included in coverage analysis as it is marked with
    /// `#[cfg(not(tarpaulin_include))]`.
    ///
    /// # Example
    /// ```rust
    /// // Hypothetical usage within a logger configuration.
    /// use crate::spotify_assistant_core::utilities::configurations::CustomFormatter;
    /// let formatter = CustomFormatter;
    /// tracing_subscriber::fmt()
    ///     .event_format(formatter)
    ///     .init();
    /// ```
    #[cfg(not(tarpaulin_include))]
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        // let level = meta.level();
        let line = meta.line().unwrap_or_default();
        let time = ChronoLocal::new("%H:%M:%S%.3f".to_string());
        let module_path_color = Color::Purple.paint(meta.target());
        let event_code_line_color = Color::Purple.bold().paint(line.to_string()).to_string();
        let log_type_color = match *meta.level() {
            Level::INFO => Color::Green.paint("INFO"),
            Level::WARN => Color::Yellow.paint("WARN"),
            Level::ERROR => Color::Red.paint("ERROR"),
            Level::DEBUG => Color::Blue.paint("DEBUG"),
            Level::TRACE => Color::White.paint("TRACE"),
        };

        time.format_time(&mut writer.by_ref())?;
        write!(
            writer,
            " [{}] {}.{} | ",
            log_type_color, module_path_color, event_code_line_color
        )?;

        // Retrieve and format the span's fields (if any)
        if let Some(scope) = ctx.event_scope() {
            let len = ctx.event_scope().unwrap().from_root().count();
            for (index, span) in scope.from_root().enumerate() {
                // Colorize the span name
                let fmt_span = Color::RGB(0, 220, 0)
                    .bold()
                    .paint(span.name().to_string())
                    .to_string();
                let fmt_arrow = Color::RGB(246, 115, 60)
                    .bold()
                    .blink()
                    .paint("->")
                    .to_string();
                // Write span name
                if index == len - 1 {
                    write!(writer, "{}: ", fmt_span)?;
                } else {
                    write!(writer, "{}{}", fmt_span, fmt_arrow)?;
                }

                // Get the formatted fields from the span
                let extensions = span.extensions();
                if let Some(fields) = extensions.get::<FormattedFields<N>>() {
                    if !fields.is_empty() {
                        // Colorize the value field from the span
                        let value_color = Color::Cyan.italic().paint(fields.to_string());
                        write!(writer, "{{{}}} ", value_color)?;
                    }
                }
            }
        }
        // Italicize the message
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
