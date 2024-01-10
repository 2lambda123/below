use std::sync::mpsc::{Receiver, TryRecvError};
use slog::Logger;
use slog::{error, warn};

pub fn handle_signals(logger: &Logger, receiver: &Receiver<Error>) -> bool {
    match receiver.try_recv() {
        Ok(e) => {
            error!(logger, "{:#}", e);
            false
        }
        Err(TryRecvError::Empty) => true,
        Err(TryRecvError::Disconnected) => {
            warn!(logger, "bpf error channel disconnected");
            true
        }
    }
}
