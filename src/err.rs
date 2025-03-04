#[derive(Debug)]
pub enum Error {
    DlOpenError,
    DlSymError,
    FindBaseAddrError,
    PlayersListError,
    TraceLineError,
}
