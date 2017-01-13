extern crate evans;
extern crate clap;

use clap::{App, Arg};
use evans::generators::Generator;
use std::str::FromStr;
use std::thread;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("evans")
        .version(VERSION.unwrap_or("unknown"))
        .version("0.1")
        .author("Brian L. Troutwine <blt@postmates.com>")
        .about("fuzz generation for telemetry servers")
        .arg(Arg::with_name("host")
            .long("host")
            .value_name("HOST")
            .takes_value(true)
            .help("the host running cernan"))
        .arg(Arg::with_name("hertz")
            .long("hertz")
            .value_name("HOST")
            .takes_value(true)
            .help("the host running cernan"))
        .arg(Arg::with_name("graphite")
            .long("graphite")
            .value_name("GRAPHITE")
            .takes_value(false)
            .help("Enable graphite fuzzing"))
        .arg(Arg::with_name("graphite_port")
            .long("graphite_port")
            .value_name("GRAPHITE_PORT")
            .takes_value(true)
            .help("graphite port"))
        .arg(Arg::with_name("statsd")
            .long("statsd")
            .value_name("STATSD")
            .takes_value(false)
            .help("Enable statsd fuzzing"))
        .arg(Arg::with_name("statsd_port")
            .long("statsd_port")
            .value_name("STATSD_PORT")
            .takes_value(true)
            .help("statsd port"))
        .arg(Arg::with_name("native")
            .long("native")
            .value_name("NATIVE")
            .takes_value(false)
            .help("Enable native fuzzing"))
        .arg(Arg::with_name("native_port")
            .long("native_port")
            .value_name("NATIVE_PORT")
            .takes_value(true)
            .help("native port"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let mut joins = Vec::new();

    let cernan_host = matches.value_of("host").unwrap_or("127.0.0.1").to_string();
    let enable_graphite = matches.is_present("graphite");
    let enable_statsd = matches.is_present("statsd");
    let enable_native = matches.is_present("native");
    let graphite_port = u16::from_str(matches.value_of("graphite_port").unwrap_or("2003")).unwrap();
    let statsd_port = u16::from_str(matches.value_of("statsd_port").unwrap_or("8125")).unwrap();
    let native_port = u16::from_str(matches.value_of("native_port").unwrap_or("1972")).unwrap();
    let hertz = u16::from_str(matches.value_of("hertz").unwrap_or("1")).unwrap();

    joins.push(thread::spawn(move || {
        evans::time::update_time();
    }));

    if enable_graphite {
        let graphite_host = cernan_host.clone();
        joins.push(thread::spawn(move || {
            evans::generators::Graphite::new(&graphite_host, graphite_port).run(hertz);
        }));
    }

    if enable_statsd {
        let statsd_host = cernan_host.clone();
        joins.push(thread::spawn(move || {
            evans::generators::Statsd::new(&statsd_host, statsd_port).run(hertz);
        }));
    }

    if enable_native {
        joins.push(thread::spawn(move || {
            evans::generators::Native::new(&cernan_host, native_port).run(hertz);
        }));
    }

    for jh in joins {
        // TODO Having sub-threads panic will not cause a bubble-up if that
        // thread is not the currently examined one. We're going to have to have
        // some manner of sub-thread communication going on.
        jh.join().expect("Uh oh, child thread paniced!");
    }
}
