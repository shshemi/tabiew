use std::sync::{
    Arc, LazyLock, Mutex,
    mpsc::{Receiver, Sender, channel},
};

use polars::frame::DataFrame;
use url::Url;

use crate::{AppResult, net::remote_load::Reader, tui::pane::TableDescription};

static SHARED_CHANNEL: LazyLock<(Sender<Message>, Mutex<Receiver<Message>>)> =
    LazyLock::new(|| {
        let (send, recv) = channel();
        (send, Mutex::new(recv))
    });

#[derive(Debug)]
pub enum Message {
    AppDismissOverlay,
    AppShowAbout,
    AppShowCommandPicker,
    AppShowError(String),
    AppShowToast(String),
    AppShowThemeSelector,
    AppShowFpPrecisionPicker,
    AppShowSchema,
    AppShowImporter,
    AppShowArrowImporter,
    AppShowAvroImporter,
    AppShowCsvImporter,
    AppShowExcelImporter,
    AppShowFwfImporter,
    AppShowHtmlImporter,
    AppShowJsonImporter,
    AppShowJsonlImporter,
    AppShowLogfmtImporter,
    AppShowMarkdownImporter,
    AppShowParquetImporter,
    AppShowSqliteImporter,
    AppShowTsvImporter,
    AppDismissSchema,
    AppShowSqlQuery,
    AppDownloadDataSource(Url, Arc<dyn Reader>),
    AppReloadConfig,
    TabsSelect(usize),
    TabsDismissSwitcher,
    TabsAddNamePane(DataFrame, String),
    TabsAddQueryPane(DataFrame, String),
    TabsCloseSelected,
    PaneEditInExternalEditor,
    PaneShowExporter,
    PaneShowArrowExporter,
    PaneShowAvroExporter,
    PaneShowCsvExporter,
    PaneShowJsonExporter,
    PaneShowJsonlExporter,
    PaneShowMarkdownExporter,
    PaneShowParquetExporter,
    PaneShowTsvExporter,
    PaneShowFuzzySearch,
    PaneShowInlineFilter,
    PaneShowInlineOrder,
    PaneShowHistogram(String, usize),
    PaneShowHistogramBuilder,
    PaneShowScatterPlot(String, String, Option<String>),
    PaneShowScatterPlotBuilder,
    PaneShowSearch,
    PaneDismissModal,
    PaneDismissSheet,
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
        let _ = SHARED_CHANNEL.0.send(self);
    }
    pub fn dequeue() -> Option<Message> {
        SHARED_CHANNEL.1.try_lock().ok()?.try_recv().ok()?.into()
    }
}

pub trait UnwrapOrEnqueueError {
    fn unwrap_or_enqueue_error(&self) -> bool;
}

impl UnwrapOrEnqueueError for AppResult<()> {
    fn unwrap_or_enqueue_error(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(err) => {
                Message::AppShowError(err.to_string()).enqueue();
                false
            }
        }
    }
}
