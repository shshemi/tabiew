use std::sync::{
    LazyLock, Mutex,
    mpsc::{Receiver, Sender, channel},
};

use polars::frame::DataFrame;

use crate::tui::pane::TableDescription;

#[derive(Debug)]
pub enum Message {
    AppDismissOverlay,
    AppShowCommandPicker,
    AppShowError(String),
    AppShowToast(String),
    AppShowThemeSelector,
    AppShowSchema,
    AppShowImporter,
    AppDismissSchema,
    AppShowSqlQuery,
    TabsSelect(usize),
    TabsDismissSwitcher,
    TabsAddNamePane(DataFrame, String),
    TabsAddQueryPane(DataFrame, String),
    PaneEditInExternalEditor,
    PaneShowExporter,
    PaneShowFuzzySearch,
    PaneShowInlineFilter,
    PaneShowInlineOrder,
    PaneShowHistogram(String, usize),
    PaneShowHistogramBuilder,
    PaneShowScatterPlot(String, String, Option<String>),
    PaneShowScatterPlotBuilder,
    PaneShowSearch,
    PaneDismissModal,
    PanePushDataFrame(DataFrame, TableDescription),
    PanePopDataFrame,
    PaneTableSelect(usize),
    PaneShowInlineSelect,
    PaneShowTableRegisterer,
    PaneShowTableInfo,
    PaneShowColumnCaster,
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
