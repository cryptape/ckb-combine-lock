use ckb_std::error::SysError;
use log::warn;
/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    // Add customized errors here...
    WrongFormat = 80,
    WrongHashType,
    ChildScriptArrayIndexOutOfBounds,
    CombineLockWitnessIndexOutOfBounds,
    UnlockFailed,
    InnerWitnessIndexOutOfBounds,
    ChildScriptHashMismatched,
    // error reported from ckb_lock_common
    // mainly from LockWrapper
    CommonError,
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
            _ => panic!("unexpected sys error"),
        }
    }
}

impl From<molecule::error::VerificationError> for Error {
    fn from(err: molecule::error::VerificationError) -> Self {
        warn!("An error reported from VerificationError: {:?}", err);
        Self::WrongFormat
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        warn!("An error reported from FromHexError: {:?}", err);
        Self::WrongFormat
    }
}

impl From<ckb_lock_common::error::Error> for Error {
    fn from(err: ckb_lock_common::error::Error) -> Self {
        warn!("An error reported from ckb_lock_common: {:?}", err);
        Self::CommonError
    }
}
