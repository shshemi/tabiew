use std::sync::{
    LazyLock, Mutex,
    mpsc::{Receiver, Sender, channel},
};

use polars::frame::DataFrame;

#[derive(Debug)]
pub enum Action {
    AppDismissOverlay,
    AppShowCommandPicker,
    AppShowError(String),
    PaneShowExportWizard,
    PaneShowInlineFilter,
    PaneShowInlineOrder,
    PaneDismissModal,
    PaneSetDataFrame(DataFrame),
    PaneTableSelectUp,
    PaneTableSelectDown,
    Quit,
}

impl Action {
    pub fn enqueue(self) {
        SHARED_CHANNEL.0.send(self).unwrap();
    }
    pub fn dequeue() -> Option<Action> {
        SHARED_CHANNEL.1.try_lock().ok()?.try_recv().ok()?.into()
    }
}

static SHARED_CHANNEL: LazyLock<(Sender<Action>, Mutex<Receiver<Action>>)> = LazyLock::new(|| {
    let (send, recv) = channel();
    (send, Mutex::new(recv))
});
