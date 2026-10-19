// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{AsyncScheduler, TimeUnits};
// Import week days and WeekDay
use crate::AppState;
use std::io;
use std::time::Duration;

use super::scan_library;

pub fn start_scheduled_indexer(state: &AppState) -> io::Result<()> {
    // Create a new scheduler
    let mut scheduler = AsyncScheduler::new();

    let config = state.get_config()?;

    let state = state.clone();

    // Add some tasks to it
    scheduler.every(10.seconds()).run(move || {
        let s = state.clone();
        async {
            match scan_library(s) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }
    });

    actix_web::rt::spawn(async move {
        actix_web::rt::time::sleep(Duration::from_secs(11)).await;
        scheduler.run_pending().await;
        loop {
            scheduler.run_pending().await;
            actix_web::rt::time::sleep(Duration::from_secs(config.scan_interval)).await;
        }
    });

    Ok(())
}
