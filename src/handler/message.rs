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
    TabsSelect(usize),
    TabsDismissSwitcher,
    PaneShowExportWizard,
    PaneShowInlineFilter,
    PaneShowInlineOrder,
    PaneShowHistogram(String, usize),
    PaneShowHistogramWizard,
    PaneShowScatterPlot(String, String, Option<String>),
    PaneShowScatterPlotWizard,
    PaneDismissModal,
    PanePushDataFrame(DataFrame),
    PanePopDataFrame,
    PaneTableSelect(usize),
    Quit,
    PaneShowInlineSelect,
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
