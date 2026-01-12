use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use midir::{MidiInput, MidiOutput, PortEvent, PortWatcher};

#[cfg(unix)]
use midir::os::unix::{VirtualInput, VirtualOutput};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(unix)]
fn run() -> Result<(), Box<dyn Error>> {
    println!("Creating port watcher...");

    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();

    let watcher = PortWatcher::new(
        "midir test watcher",
        move |event, _: &mut ()| {
            let mut events = events_clone.lock().unwrap();
            events.push(format!("{:?}", event));
            match event {
                PortEvent::InputAdded(port) => {
                    println!("[Event #{}] Input port added: {}", events.len(), port.id());
                }
                PortEvent::InputRemoved(port) => {
                    println!(
                        "[Event #{}] Input port removed: {}",
                        events.len(),
                        port.id()
                    );
                }
                PortEvent::OutputAdded(port) => {
                    println!("[Event #{}] Output port added: {}", events.len(), port.id());
                }
                PortEvent::OutputRemoved(port) => {
                    println!(
                        "[Event #{}] Output port removed: {}",
                        events.len(),
                        port.id()
                    );
                }
            }
        },
        (),
    )?;

    println!("Watcher created successfully.\n");
    println!("Testing with virtual MIDI ports...\n");

    // Give the watcher thread time to start
    thread::sleep(Duration::from_millis(100));

    println!("Creating virtual input port...");
    let midi_in = MidiInput::new("test input")?;
    let in_conn = midi_in.create_virtual("Virtual Test Input", |_, _, _| {}, ())?;
    thread::sleep(Duration::from_millis(200));

    println!("Creating virtual output port...");
    let midi_out = MidiOutput::new("test output")?;
    let out_conn = midi_out.create_virtual("Virtual Test Output")?;
    thread::sleep(Duration::from_millis(200));

    println!("Closing virtual output port...");
    let _midi_out = out_conn.close();
    thread::sleep(Duration::from_millis(200));

    println!("Closing virtual input port...");
    let (_midi_in, _) = in_conn.close();
    thread::sleep(Duration::from_millis(200));

    println!("\nStopping watcher...");
    watcher.stop();

    let events = events.lock().unwrap();
    println!("\nTotal events received: {}", events.len());

    if events.len() > 0 {
        println!("\nAll events:");
        for (i, event) in events.iter().enumerate() {
            println!("  {}. {}", i + 1, event);
        }
        println!(
            "\n✓ Port watcher test PASSED - received {} events",
            events.len()
        );
    } else {
        println!("\n✗ Port watcher test FAILED - no events received");
        println!("This might be expected on some systems where virtual ports don't trigger notifications");
    }

    Ok(())
}

#[cfg(not(unix))]
fn run() -> Result<(), Box<dyn Error>> {
    println!("Virtual port test is only available on Unix systems (Linux, macOS)");
    println!(
        "Please run the basic test_port_watcher example and manually plug/unplug a MIDI device."
    );
    Ok(())
}
