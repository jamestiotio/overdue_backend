// Define convenience facilities for logging purposes
use slog::{Drain, Filter, Level, Logger, Record, o};
use slog_term::{TermDecorator, FullFormat};
use slog_async::{Async, OverflowStrategy};
use std::io;
use std::sync::atomic::Ordering;
use chrono::Local;

use crate::constants::LEVEL;

pub fn set_global_level(level: Level) {
    LEVEL.store(level.as_usize(), Ordering::SeqCst);
}

fn timestamp_fn(io: &mut dyn io::Write) -> io::Result<()> {
    write!(
        io,
        "{}",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    )
}

pub fn configure_log() -> Logger {
    // Set logging settings for debugging purposes
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator)
        .use_custom_timestamp(timestamp_fn)
        .build()
        .fuse();
    let drain = Async::new(drain)
        .chan_size(1_024)
        .overflow_strategy(OverflowStrategy::Block)
        .build()
        .fuse();
    let drain = Filter::new(drain, |r: &Record| {
        let level = LEVEL.load(Ordering::Relaxed);
        let level = Level::from_usize(level).expect("error getting an invalid level usize");
        r.level().is_at_least(level)
    });
    Logger::root(drain.fuse(), o!("v" => env!("CARGO_PKG_VERSION")))
}