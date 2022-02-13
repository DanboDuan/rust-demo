use ctrlc;
use signal_hook::consts::signal::*;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::io::{self, BufRead};
use std::process;
use std::sync::mpsc::channel;
use std::thread;

use std::error::Error;

fn handle_ctrlc() {
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    thread::spawn(move || {
        rx.recv().expect("Could not receive Ctrl-C  from channel.");
        println!("Got it! Exiting...");
        process::exit(1);
    });
}

fn handle_more() -> Result<(), Box<dyn Error>> {
    let mut sigs = vec![
        // Some terminal handling
        SIGTSTP, SIGCONT, SIGWINCH,
        // Reload of configuration for daemons ‒ um, is this example for a TUI app or a daemon
        // O:-)? You choose...
        SIGHUP, // Application-specific action, to print some statistics.
        SIGUSR1,
    ];
    sigs.extend(TERM_SIGNALS);
    let mut signals = Signals::new(&sigs)?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });
    Ok(())
}

fn main() {
    handle_ctrlc();
    handle_more().expect("handle more error");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => {
                eprintln!("std readline error");
                break;
            }
            Ok(s) => println!("input --> {}", s),
        }
    }
    println!("fin");
}
