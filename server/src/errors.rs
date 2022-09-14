use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("list of quotes is empty")]
    EmptyQuotesList,
}
