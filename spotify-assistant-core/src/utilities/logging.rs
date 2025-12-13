use ansi_term::{ANSIGenericString, Color};
use std::collections::VecDeque;
use std::io;
use tracing::{Level, Metadata};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{ChronoLocal, FormatTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

static _LOG_GUARD: once_cell::sync::OnceCell<tracing_appender::non_blocking::WorkerGuard> =
    once_cell::sync::OnceCell::new();

pub fn init_tracing() {
    const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    // Base filter: crate only, everything else off.
    let base_directive = EnvFilter::new(format!(
        "off,{crate}=trace,testing=trace",
        crate = CRATE_NAME
    ));
    // Console filter: crate only, everything else off.
    // In debug: TRACE, in release: INFO.
    let console_directive = if cfg!(debug_assertions) {
        format!("off,{crate}=trace,testing=trace", crate = CRATE_NAME)
    } else {
        format!("off,{crate}=info", crate = CRATE_NAME)
    };

    // Console: human-friendly formatting, your existing formatter, INFO+ only
    let console_layer = fmt::layer()
        .event_format(CustomDevFormatter)
        .with_writer(io::stderr)
        .with_filter(LevelFilter::TRACE);

    #[cfg(debug_assertions)]
    tracing_subscriber::registry()
        .with(base_directive) // global filtering (dependency logging: off)
        .with(console_layer.with_filter(EnvFilter::new(console_directive))) // TRACE+ without dependencies to console
        .init();

    #[cfg(not(debug_assertions))]
    {
        // File appender (logs/countycrawler.log, rotated daily)
        let file_appender = rolling::daily("logs", "spotifyassistant.log");
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

        // Store guard so async threads don't drop
        let _ = _LOG_GUARD.set(guard);

        // File: standard format is fine, DEBUG+ (controlled by EnvFilter above)
        let file_layer = fmt::layer().with_ansi(false).with_writer(file_writer);

        tracing_subscriber::registry()
            .with(base_directive) // global filtering (dependency logging: off)
            .with(console_layer.with_filter(EnvFilter::new(console_directive))) // TRACE+ without dependencies to console
            .with(file_layer.with_filter(LevelFilter::TRACE)) // full detail to logs/countycrawler.log
            .init();
    }

    tracing::debug!("Tracing subscriber initialized");
}
pub struct CustomDevFormatter;

impl CustomDevFormatter {
    fn log_colors(&self, meta: &Metadata) -> ANSIGenericString<'_, str> {
        match *meta.level() {
            Level::INFO => Color::Green.paint("INFO"),
            Level::WARN => Color::Yellow.paint("WARN"),
            Level::ERROR => Color::Red.paint("ERROR"),
            Level::DEBUG => Color::Blue.paint("DEBUG"),
            Level::TRACE => Color::White.paint("TRACE"),
        }
    }
    fn filename(&self, meta: &Metadata) -> String {
        let file = meta.file().unwrap_or("?");
        file.split('/').last().unwrap_or("?").to_string()
    }
    fn line_color(&self, meta: &Metadata) -> String {
        let line_str = meta
            .line()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "?".to_string());
        Color::Purple.bold().paint(&line_str).to_string()
    }
    fn file_path_color(&self, meta: &Metadata, file_name: &str) -> String {
        let mut module_path_vec = meta.target().split("::").collect::<VecDeque<&str>>();
        let _ = module_path_vec.pop_back();
        let _ = module_path_vec.pop_front();
        module_path_vec.insert(0, "src");
        module_path_vec.insert(module_path_vec.len(), file_name);
        let file_path = module_path_vec
            .iter()
            .map(|module| module.to_string())
            .collect::<Vec<String>>()
            .join(".");
        Color::Purple.paint(file_path).to_string()
    }
}

impl<S, N> FormatEvent<S, N> for CustomDevFormatter
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
    /// use crate::spotify_assistant_core::utilities::logging::CustomDevFormatter;
    /// let formatter = CustomDevFormatter;
    /// tracing_subscriber::fmt()
    ///     .event_format(formatter)
    ///     .init();
    /// ```
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();
        let log_type_color = self.log_colors(meta);
        let filename = self.filename(meta);
        let file_path = self.file_path_color(meta, &filename);
        let event_code_line_color = self.line_color(meta);
        let time = ChronoLocal::new("%H:%M:%S%.3f".to_string());

        time.format_time(&mut writer.by_ref())?;
        write!(
            writer,
            " [{}] {}:{} | ",
            log_type_color, file_path, event_code_line_color
        )?;

        // Retrieve and format the span's fields (if any)
        if let Some(scope) = ctx.event_scope() {
            let mut iter = scope.from_root().peekable();
            while let Some(span) = iter.next() {
                let fmt_span = Color::RGB(0, 220, 0).bold().paint(span.name()).to_string();
                let fmt_arrow = Color::RGB(246, 115, 60).bold().paint("->").to_string();

                if iter.peek().is_some() {
                    write!(writer, "{}{}", fmt_span, fmt_arrow)?;
                } else {
                    write!(writer, "{}: ", fmt_span)?;
                }

                let extensions = span.extensions();
                if let Some(fields) = extensions.get::<FormattedFields<N>>() {
                    if !fields.is_empty() {
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
