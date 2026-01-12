use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{PortEvent, PortWatcher};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Creating port watcher...");

    let watcher = PortWatcher::new(
        "midir test watcher",
        |event, count: &mut u32| {
            *count += 1;
            match event {
                PortEvent::InputAdded(port) => {
                    println!("[Event #{}] Input port added: {}", count, port.id());
                }
                PortEvent::InputRemoved(port) => {
                    println!("[Event #{}] Input port removed: {}", count, port.id());
                }
                PortEvent::OutputAdded(port) => {
                    println!("[Event #{}] Output port added: {}", count, port.id());
                }
                PortEvent::OutputRemoved(port) => {
                    println!("[Event #{}] Output port removed: {}", count, port.id());
                }
            }
        },
        0u32,
    )?;

    println!("Watching for MIDI port changes...");
    println!("Try connecting or disconnecting MIDI devices.");
    println!("Press <enter> to stop watching...\n");

    let mut input = String::new();
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    println!("\nStopping watcher...");
    let event_count = watcher.stop();
    println!("Total events received: {}", event_count);

    Ok(())
}
