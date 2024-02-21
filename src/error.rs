#[derive(Debug)]
pub enum Error {
    // region    : --- Request
    ParseBody,
    ParseHeading,
    // endregion : --- Request

    // region    : --- Header
    ReadHeader,
    // endregion : --- Header

    // region    : --- Little Hyper
    Listen,
    ReadStreamInfo
    // endregion : --- Little Hyper

}

pub type Result<T> = core::result::Result<T, Error>;
