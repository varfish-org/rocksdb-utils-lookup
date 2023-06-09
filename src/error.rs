//! Error type definition.

use thiserror::Error;

/// Error type for `rocksdb-utils-lookup`
#[derive(Error, Debug)]
pub enum Error {
    /// Problem opening RocksDB.
    #[error("problem opening RocksDB at {0}: {1}")]
    Open(std::path::PathBuf, #[source] rocksdb::Error),
    /// Problem with RocksDB property query.
    #[error("problem accessing RocksDB property: {0}")]
    PropertyAccess(#[source] rocksdb::Error),
    /// The RocksDB property was not set.
    #[error("RocksDB property {0} was not set")]
    PropertyNotSet(String),
    /// Problem with acessing RocksDB column family.
    #[error("problem accessing RocksDB column family: {0}")]
    ColumnFamily(String),
    /// Problem with directory access or manipulation in WAL removal.
    #[error("problem with directory access/manipulation in WAL removal: {0}")]
    WalRemoval(#[source] std::io::Error),
}
