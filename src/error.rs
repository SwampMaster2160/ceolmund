use std::fmt::{Display, Debug};
use std::error;

/// A game error.
pub enum Error {
	CannotCreateFolder,
	CannotSaveNamespace,
	CannotReadNamespace,
	CannotReadWorldOverview,
	OutOfBoundsFileRead,
	FileStringReadError,
	IDOutOfNamespaceBounds,
	IDOutOfMetaNamespaceBounds,
	CannotDeleteFile,
	CannotRenameFile,
	CannotReadFile,
	UnterminatedStringRead,
	InvalidUTF8InString,
	FutureSerializationVersion,
	InvalidNamespaceName,
	
	InvalidString,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CannotCreateFolder => write!(f, "Cannot create folder."),
			Self::CannotSaveNamespace => write!(f, "Cannot save namespace."),
			Self::CannotReadNamespace => write!(f, "Cannot read namespace."),
			Self::CannotReadWorldOverview => write!(f, "Cannot read world overview."),
			Self::OutOfBoundsFileRead => write!(f, "Out of bounds file read."),
			Self::FileStringReadError => write!(f, "Error reading string from file."),
			Self::IDOutOfNamespaceBounds => write!(f, "ID is out of namespace bounds."),
			Self::CannotDeleteFile => write!(f, "Cannot delete file."),
			Self::CannotRenameFile => write!(f, "Cannot rename file."),
			Self::CannotReadFile => write!(f, "Cannot read file."),
			Self::UnterminatedStringRead => write!(f, "Unterminated string read."),
			Self::InvalidUTF8InString =>  write!(f, "The string contains a invalid UTF-8 byte sequence."),
			Self::FutureSerializationVersion => write!(f, "Future serialization version encountered."),
			Self::InvalidString => write!(f, "Invalid string."),
			Self::IDOutOfMetaNamespaceBounds => write!(f, "Invalid namespace ID."),
			Self::InvalidNamespaceName => write!(f, "Invalid namespace name."),
		}
	}
}

impl Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl error::Error for Error {

}