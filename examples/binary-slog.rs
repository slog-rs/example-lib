/// An example of an application, using a library that supports `slog` logging.
///
/// In this case the application does provide a `Logger` and the library will use it.
extern crate slog_example_lib;
extern crate slog_term;
#[macro_use]
extern crate slog;
extern crate slog_async;

use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("version" => "0.5"));

    let lib = slog_example_lib::MyLib::init(log);
    lib.do_the_thing();
}
