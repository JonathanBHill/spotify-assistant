use ansi_term::Color;
use tracing::Level;
use tracing_subscriber::fmt::{FmtContext, format::Writer, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::fmt::time::{ChronoLocal, FormatTime};
use tracing_subscriber::registry::LookupSpan;

pub struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
        N: for<'a> FormatFields<'a> + 'static,
{
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
        write!(writer, " [{}] {}.{} | ", log_type_color, module_path_color, event_code_line_color)?;

        // Retrieve and format the span's fields (if any)
        if let Some(scope) = ctx.event_scope() {
            let len = ctx.event_scope().unwrap().from_root().count();
            for (index, span) in scope.from_root().enumerate() {
                // Colorize the span name
                let fmt_span = Color::RGB(0,220,0).bold().paint(span.name().to_string()).to_string();
                let fmt_arrow = Color::RGB(246, 115, 60).bold().blink().paint("->").to_string();
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
