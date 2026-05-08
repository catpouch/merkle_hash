use std::cmp::Ordering;

use crate::components::merkle_path::MerklePath;

#[cfg(feature = "entry-kind")]
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub enum EntryKind {
    File,
    Directory,
    Unknown
}

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub struct MerkleItem {
    pub path: MerklePath,
    pub hash: Vec<u8>,
    #[cfg(feature = "entry-kind")]
    pub kind: EntryKind,
    #[cfg(feature = "retain")]
    pub children_paths: std::collections::BTreeSet<MerklePath>,
}

impl MerkleItem {
    pub fn new(
        path: MerklePath,
        hash: Vec<u8>,
        #[cfg(feature = "entry-kind")]
        kind: EntryKind,
        #[cfg(feature = "retain")]
        children_paths: std::collections::BTreeSet<MerklePath>
    ) -> Self {
        Self {
            path,
            hash,
            #[cfg(feature = "entry-kind")]
            kind,
            #[cfg(feature = "retain")]
            children_paths
        }
    }
}

impl PartialOrd<Self> for MerkleItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MerkleItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
