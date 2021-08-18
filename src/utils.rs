use cfg_if::cfg_if;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn md5sum(buf: &str) -> String { format!("{:x}", md5::compute(buf)) }

pub fn base64(buf: &str) -> String { base64::encode(buf) }

pub fn month() -> u32 {
    use chrono::{Utc, Datelike};
    let now = Utc::now();
    now.month()
}

#[macro_export]
macro_rules! check {
    ($form: ident, $passwd: expr, $allow_token: expr) => {
        if !$form.auth($passwd, $allow_token) {
            return Ok(crate::http::forbidden());
        }
        if $form.proto.is_none() {
            return Ok(crate::http::not_found());
        }
    };
}
