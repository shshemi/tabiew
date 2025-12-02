use std::sync::{
    LazyLock, Mutex,
    mpsc::{Receiver, Sender, channel},
};

use polars::frame::DataFrame;

#[derive(Debug)]
pub enum Message {
    AppDismissOverlay,
    AppShowCommandPicker,
    AppShowError(String),
    AppShowThemeSelector,
    PaneShowExportWizard,
    PaneShowInlineFilter,
    PaneShowInlineOrder,
    PaneDismissModal,
    PaneSetDataFrame(DataFrame),
    PaneTableSelectUp,
    PaneTableSelectDown,
    PaneTableSelect(usize),
    Quit,
}

impl Message {
    pub fn enqueue(self) {
        SHARED_CHANNEL.0.send(self).unwrap();
    }
    pub fn dequeue() -> Option<Message> {
        SHARED_CHANNEL.1.try_lock().ok()?.try_recv().ok()?.into()
    }
}

static SHARED_CHANNEL: LazyLock<(Sender<Message>, Mutex<Receiver<Message>>)> =
    LazyLock::new(|| {
        let (send, recv) = channel();
        (send, Mutex::new(recv))
    });
