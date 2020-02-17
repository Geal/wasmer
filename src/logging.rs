use crate::utils::wasmer_should_print_color;
use fern::colors::{Color, ColoredLevelConfig};
use std::time;

/// Subroutine to instantiate the loggers
pub fn set_up_logging() -> Result<(), String> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .trace(Color::BrightBlack);
    let should_color = wasmer_should_print_color();

    let colors_level = colors_line.info(Color::Green);
    let dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain({
            let base = if should_color {
                fern::Dispatch::new().format(move |out, message, record| {
                    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Can't get time");
                    out.finish(format_args!(
                        "{color_line}[{seconds}.{millis} {level} {target}{color_line}]{ansi_close} {message}",
                        color_line = format_args!(
                            "\x1B[{}m",
                            colors_line.get_color(&record.level()).to_fg_str()
                        ),
                        seconds = time.as_secs(),
                        millis = time.subsec_millis(),
                        level = colors_level.color(record.level()),
                        target = record.target(),
                        ansi_close = "\x1B[0m",
                        message = message,
                    ));
                })
            } else {
                // default formatter without color
                fern::Dispatch::new().format(move |out, message, record| {
                    let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Can't get time");
                    out.finish(format_args!(
                        "[{seconds}.{millis} {level} {target}] {message}",
                        seconds = time.as_secs(),
                        millis = time.subsec_millis(),
                        level = record.level(),
                        target = record.target(),
                        message = message,
                    ));
                })
            };

            base.chain(std::io::stdout())
        });

    dispatch.apply().map_err(|e| format!("{}", e))?;

    Ok(())
}
