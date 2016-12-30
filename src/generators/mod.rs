mod graphite;
mod native;
mod statsd;

pub use self::graphite::Graphite;
pub use self::native::Native;
pub use self::statsd::Statsd;

pub trait Generator {
    fn run(&self, hertz: u16) -> ();
}
