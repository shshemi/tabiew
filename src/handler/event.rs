use crate::AppResult;
use crate::misc::type_ext::UnwrapOrGracefulShutdown;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::{Mutex, MutexGuard, OnceLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

pub fn lock_event() -> MutexGuard<'static, ()> {
    static EVENT_MUTEX: Mutex<()> = Mutex::new(());
    EVENT_MUTEX.lock().unwrap_or_graceful_shutdown()
}

pub fn read_event() -> AppResult<Event> {
    static RECV: OnceLock<Mutex<EventHandler>> = OnceLock::new();
    RECV.get_or_init(|| Mutex::new(EventHandler::new(100)))
        .lock()
        .unwrap_or_graceful_shutdown()
        .next()
}

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
struct EventHandler {
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                {
                    let _lock = lock_event();

                    if event::poll(timeout).expect("failed to poll new events") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            CrosstermEvent::FocusGained => Ok(()),
                            CrosstermEvent::FocusLost => Ok(()),
                            CrosstermEvent::Paste(_) => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if sender.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });
        Self { receiver }
    }

    fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
