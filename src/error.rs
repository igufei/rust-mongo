use std::fmt::Display;

pub struct Error(pub &'static str);

pub trait ResultExtention<T> {
    fn has_err(self, err_text: &'static str) -> Result<T, Error>;
}

impl<T, E> ResultExtention<T> for Result<T, E>
where
    E: Display,
{
    fn has_err(self, err_text: &'static str) -> Result<T, Error> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                println!("{}",e);
                Err(Error(err_text))
            }
        }
    }
}
