use generators::Generator;
use rand;
use rand::Rng;
use std::cmp;
use std::io::{BufWriter, Write};
use std::net;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::string;
use time;

pub struct Graphite {
    addr: net::SocketAddr,
}

impl Graphite {
    pub fn new(host: &str, port: u16) -> Graphite {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
        Graphite { addr: addr }
    }
}

#[inline]
fn get_from_cache<T>(cache: &mut Vec<(T, String)>, val: T) -> &str
    where T: cmp::PartialOrd + string::ToString + Copy
{
    match cache.binary_search_by(|probe| probe.0.partial_cmp(&val).unwrap()) {
        Ok(idx) => &cache[idx].1,
        Err(idx) => {
            let str_val = val.to_string();
            cache.insert(idx, (val, str_val));
            get_from_cache(cache, val)
        }
    }
}

static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

impl Generator for Graphite {
    fn run(&self, hertz: u16) -> () {
        let mut time_cache: Vec<(i64, String)> = Vec::with_capacity(128);
        let mut value_cache: Vec<(f64, String)> = Vec::with_capacity(128);

        let mut name_cache: Vec<String> = Vec::new();
        for a in &ASCII_LOWER {
            for b in &ASCII_LOWER {
                for c in &ASCII_LOWER {
                    name_cache.push(format!("{}{}{}", a, b, c));
                }
            }
        }

        let mut rng = rand::thread_rng();

        let mut line = String::new();
        let mut stream = BufWriter::new(TcpStream::connect(self.addr).unwrap());
        loop {
            let metric_value: f64 = rng.gen::<f64>();
            let offset: i64 = rng.gen_range(-10, 10);

            line.push_str(rng.choose(&name_cache).unwrap());
            line.push_str(" ");
            line.push_str(get_from_cache(&mut value_cache, metric_value));
            line.push_str(" ");
            line.push_str(get_from_cache(&mut time_cache, time::now() + offset));
            line.push_str("\n");

            stream.write(line.as_bytes()).unwrap();

            line.clear();
            if time_cache.len() > 1_000 {
                time_cache.clear();
            }
            time::sleep_hertz(hertz);
        }
    }
}
