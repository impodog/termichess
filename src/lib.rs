pub mod game;
pub mod local;
pub mod prelude;
pub mod remote;
pub mod util;

lazy_static::lazy_static! {
    pub static ref CONFIG: util::Config = util::get_config().unwrap().modify();
}
