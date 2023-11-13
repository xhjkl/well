//! Progress indicator.
use colored::Colorize;
use std::thread::JoinHandle;

type Signal = std::sync::Arc<std::sync::atomic::AtomicBool>;

pub struct Throbber(Option<(Signal, JoinHandle<()>)>);

impl Throbber {
    pub fn stop(self) {
        let Some((signal, handle)) = self.0 else {
            return;
        };

        signal.store(true, std::sync::atomic::Ordering::Relaxed);
        handle.join().unwrap();
    }
}

/// Show a pseudo-textual progress indicator until the given signal is set.
fn indicate_until_signalled(signal: &Signal) {
    let frames = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏";
    let mut i = 0;
    loop {
        if signal.load(std::sync::atomic::Ordering::Relaxed) {
            eprint!("\r \r");
            break;
        }
        let frame = frames
            .chars()
            .nth(i % frames.len())
            .unwrap_or_default()
            .to_string();
        eprint!("\r{}", frame.bright_cyan());
        i += 1;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Start showing and updating a progress indicator
/// in a separate thread.
#[must_use]
pub fn start_throbber() -> Throbber {
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        return Throbber(None);
    }

    let signal = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let other_signal = signal.clone();
    let handle = std::thread::spawn(move || {
        indicate_until_signalled(&other_signal);
    });
    Throbber(Some((signal, handle)))
}
