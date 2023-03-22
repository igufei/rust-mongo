

pub struct Error(pub &'static str);

pub trait ResultExtention<T> {
    fn has_err(self, err_text: &'static str) -> Result<T, Error>;
}

impl<T, E> ResultExtention<T> for Result<T, E> {
    fn has_err(self, err_text: &'static str) -> Result<T, Error> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(Error(err_text)),
        }
    }
}
