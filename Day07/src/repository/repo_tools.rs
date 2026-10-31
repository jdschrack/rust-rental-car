pub(crate) type RepoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn invalid_row(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message.to_owned())
}
