use std::fmt::Display;
/// mongodb error
pub struct Error(pub &'static str);


pub trait ResultExtention<T> {
    fn has_err(self, err_text: &'static str) -> Result<T, Error>;
}

impl<T, E> ResultExtention<T> for Result<T, E>
where
    E: Display,
{
    /// 错误处理
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
