use duckdb::{Connection, ProgressHandle};
use std::path::{Path, PathBuf};

pub enum ConnectionType {
    ConnectionInMem,
    ConnectionFile,
}

pub struct ConnectionManager {
    conn: Connection,
    conn_type: ConnectionType,
    progress: ProgressHandle,
    file_path: Option<PathBuf>,
}
