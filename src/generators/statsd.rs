use generators::Generator;
use rand;
use rand::Rng;
use std::cmp;
use std::net;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::string;
use time;

pub struct Statsd {
    addr: net::SocketAddr,
    dest: net::SocketAddr,
}

impl Statsd {
    pub fn new(host: &str, port: u16) -> Statsd {
        let addr = ("0.0.0.0", 0).to_socket_addrs().unwrap().next().unwrap();
        let dest = (host, port).to_socket_addrs().unwrap().next().unwrap();
        Statsd {
            addr: addr,
            dest: dest,
        }
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

impl Generator for Statsd {
    fn run(&self, hertz: u16) -> () {
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
        let types = ["c", "ms", "h", "g"];

        let mut line = String::new();
        let socket = UdpSocket::bind(self.addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        loop {
            let metric_value: f64 = rng.gen::<f64>();
            let metric_type: &str = rng.choose(&types).unwrap();

            line.push_str(rng.choose(&name_cache).unwrap());
            line.push_str(":");
            line.push_str(get_from_cache(&mut value_cache, metric_value));
            line.push_str("|");
            line.push_str(metric_type);
            line.push_str("\n");

            socket.send_to(line.as_bytes(), self.dest).unwrap();

            line.clear();
            time::sleep_hertz(hertz);
        }
    }
}
