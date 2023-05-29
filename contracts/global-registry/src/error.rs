use ckb_std::error::SysError;
use log::warn;

/// Error
#[repr(i8)]
#[derive(Debug)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    InvalidInitHash = 50,
    TransformingError,
    OutputTypeForbidden,
    InvalidLinkedList,
    UpdateFailed,
    LockScriptNotExisting,
    LockScriptDup,
    NotMatchingLockScript,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<ckb_combine_lock_common::error::Error> for Error {
    fn from(err: ckb_combine_lock_common::error::Error) -> Self {
        warn!("An error reported from ckb_combine_lock_common: {:?}", err);
        Self::TransformingError
    }
}