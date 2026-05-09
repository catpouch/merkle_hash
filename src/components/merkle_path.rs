use std::cmp::Ordering;

/// Represents the type of entry that a MerklePath points to (file, directory, or other)
#[cfg(feature = "entry-kind")]
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub enum EntryKind {
    File,
    Directory,
    Unknown
}

#[cfg(feature = "entry-kind")]
impl EntryKind {
    /// Returns the entry kind of the supplied path
    pub fn from_path<T: AsRef<std::path::Path>>(path: &T) -> Self {
        let path = path.as_ref();
        if path.is_file() {
            crate::components::merkle_path::EntryKind::File
        } else if path.is_dir() {
            crate::components::merkle_path::EntryKind::Directory
        } else {
            crate::components::merkle_path::EntryKind::Unknown
        }
    }
}

/// A utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub struct MerklePath {
    #[cfg(feature = "camino")]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub relative: camino::Utf8PathBuf,
    #[cfg(feature = "camino")]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub absolute: camino::Utf8PathBuf,
    
    #[cfg(not(feature = "camino"))]
    pub relative: std::path::PathBuf,
    #[cfg(not(feature = "camino"))]
    pub absolute: std::path::PathBuf,

    #[cfg(feature = "entry-kind")]
    pub kind: EntryKind
}

impl MerklePath {
    pub fn new(
        #[cfg(feature = "camino")]
        relative_path: camino::Utf8PathBuf,
        #[cfg(feature = "camino")]
        absolute_path: camino::Utf8PathBuf,

        #[cfg(not(feature = "camino"))]
        relative_path: std::path::PathBuf,
        #[cfg(not(feature = "camino"))]
        absolute_path: std::path::PathBuf,

        #[cfg(feature = "entry-kind")]
        kind: EntryKind
    ) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
            #[cfg(feature = "entry-kind")]
            kind
        }
    }
}

impl PartialOrd<Self> for MerklePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MerklePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative.cmp(&other.relative)
    }
}
