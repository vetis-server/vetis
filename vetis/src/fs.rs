use std::path::PathBuf;

/// Builder for creating FileMetadata instances
pub struct FileMetadataBuilder {
    mime: Option<String>,
    size: u64,
    modified: std::time::SystemTime,
    etag: Option<String>,
}

impl FileMetadataBuilder {
    /// Set the MIME type of the file
    pub fn mime(mut self, mime: Option<String>) -> Self {
        self.mime = mime;
        self
    }

    /// Set the size of the file
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    /// Set the last modified time of the file
    pub fn modified(mut self, modified: std::time::SystemTime) -> Self {
        self.modified = modified;
        self
    }

    /// Set the ETag of the file
    pub fn etag(mut self, etag: Option<String>) -> Self {
        self.etag = etag;
        self
    }

    /// Build the FileMetadata instance
    pub fn build(self) -> FileMetadata {
        FileMetadata { mime: self.mime, size: self.size, modified: self.modified, etag: self.etag }
    }
}

/// Metadata for a file
#[derive(Clone)]
pub struct FileMetadata {
    mime: Option<String>,
    size: u64,
    modified: std::time::SystemTime,
    etag: Option<String>,
}

impl FileMetadata {
    /// Create a new FileMetadataBuilder
    pub fn builder() -> FileMetadataBuilder {
        FileMetadataBuilder {
            mime: None,
            size: 0,
            modified: std::time::SystemTime::now(),
            etag: None,
        }
    }

    /// Get the MIME type of the file
    pub fn mime(&self) -> Option<&String> {
        self.mime.as_ref()
    }

    /// Get the size of the file
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get the last modified time of the file
    pub fn modified(&self) -> std::time::SystemTime {
        self.modified
    }

    /// Get the ETag of the file
    pub fn etag(&self) -> Option<&String> {
        self.etag.as_ref()
    }
}

pub trait File {
    fn metadata(&self) -> &FileMetadata;
    fn data(&self) -> Option<&Vec<u8>>;
    fn path(&self) -> Option<&PathBuf>;
}

/// Represents a file that can be either in memory or on disk
#[derive(Clone)]
pub enum FileSource {
    /// File data in memory
    Data {
        /// The file data
        data: Vec<u8>,
        /// The file metadata
        metadata: FileMetadata,
    },
    /// File path on disk
    Path {
        /// The file path
        path: PathBuf,
        /// The file metadata
        metadata: FileMetadata,
    },
}

impl File for FileSource {
    /// Get the metadata for the file
    fn metadata(&self) -> &FileMetadata {
        match self {
            FileSource::Data { metadata, .. } => metadata,
            FileSource::Path { metadata, .. } => metadata,
        }
    }

    /// Get the data for the file (if in memory)
    fn data(&self) -> Option<&Vec<u8>> {
        match self {
            FileSource::Data { data, .. } => Some(data),
            FileSource::Path { .. } => None,
        }
    }

    /// Get the path for the file (if on disk)
    fn path(&self) -> Option<&PathBuf> {
        match self {
            FileSource::Data { .. } => None,
            FileSource::Path { path, .. } => Some(path),
        }
    }
}
