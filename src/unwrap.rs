use crate::log;

pub trait Custom<T> {
    fn unwrap_log(self);
    fn expect_log(self, msg: &str);
}

impl<T, E> Custom<T> for Result<T, E>
where
    E: ToString,
{
    fn unwrap_log(self) {
        match self {
            Ok(_) => (),
            Err(e) => log::error!("{}", e.to_string()),
        }
    }

    fn expect_log(self, msg: &str) {
        match self {
            Ok(_) => (),
            Err(e) => log::error!("{}: {}", msg, e.to_string()),
        }
    }
}
