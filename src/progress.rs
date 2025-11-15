// ! Progress indicators for long-running operations

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a spinner for indeterminate operations
pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .expect("Invalid spinner template - this is a bug"),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Create a progress bar for determinate operations
#[allow(dead_code)]
pub fn progress_bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} {elapsed_precise}")
            .expect("Invalid progress bar template - this is a bug")
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Create a multi-progress container for parallel operations
#[allow(dead_code)]
pub fn multi_progress() -> MultiProgress {
    MultiProgress::new()
}

/// Finish progress bar with success message
pub fn finish_success(pb: &ProgressBar, msg: &str) {
    pb.finish_with_message(format!("✓ {}", msg));
}

/// Finish progress bar with error message
pub fn finish_error(pb: &ProgressBar, msg: &str) {
    pb.finish_with_message(format!("✗ {}", msg));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let sp = spinner("Testing...");
        assert!(!sp.is_finished());
        sp.finish();
        assert!(sp.is_finished());
    }

    #[test]
    fn test_progress_bar_creation() {
        let pb = progress_bar(100, "Processing");
        assert!(!pb.is_finished());
        pb.inc(50);
        assert_eq!(pb.position(), 50);
        pb.finish();
        assert!(pb.is_finished());
    }

    #[test]
    fn test_finish_messages() {
        let pb = spinner("Working");
        finish_success(&pb, "Completed");
        assert!(pb.is_finished());

        let pb2 = spinner("Failing");
        finish_error(&pb2, "Failed");
        assert!(pb2.is_finished());
    }
}
