use byteorder::{BigEndian, ByteOrder};
use generators::Generator;
use protobuf::Message;
use protobuf::repeated::RepeatedField;
use protocols::native::{AggregationMethod, LogLine, LogLine_MetadataEntry, Payload, Telemetry,
                        Telemetry_MetadataEntry};
use rand;
use rand::Rng;
use std::io::BufWriter;
use std::io::Write;
use std::net;
use std::net::{TcpStream, ToSocketAddrs};
use time;

pub struct Native {
    addr: net::SocketAddr,
}

impl Native {
    pub fn new(host: &str, port: u16) -> Native {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
        Native { addr: addr }
    }
}

static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                  'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

static AGGR_METHODS: [AggregationMethod; 3] =
    [AggregationMethod::SET, AggregationMethod::SUM, AggregationMethod::SUMMARIZE];

static META_KEYS: [&'static str; 4] = ["one", "two", "three", "four"];

static META_VALS: [&'static str; 4] = ["eins", "zwei", "drei", "vier"];

impl Generator for Native {
    fn run(&self, hertz: u16) -> () {
        let mut rng = rand::thread_rng();
        let mut bufwrite = BufWriter::new(TcpStream::connect(self.addr).unwrap());
        let mut sz_buf = [0; 4];

        let mut name_cache: Vec<String> = Vec::new();
        for a in &ASCII_LOWER {
            for b in &ASCII_LOWER {
                for c in &ASCII_LOWER {
                    name_cache.push(format!("{}{}{}", a, b, c));
                }
            }
        }

        loop {
            let mut pyld = Payload::new();

            let mut points = Vec::with_capacity(1024);
            for _ in 0..rng.gen_range::<usize>(0, 100) {
                let mut telem = Telemetry::new();
                telem.set_name(rng.choose(&name_cache).unwrap().to_string());
                let smpl_top = rng.gen_range::<usize>(0, 10);
                let mut smpls = Vec::with_capacity(smpl_top);
                for _ in 0..smpl_top {
                    smpls.push(rng.gen::<f64>())
                }
                telem.set_samples(smpls);
                telem.set_method(*rng.choose(&AGGR_METHODS).unwrap());
                let mut metadata = Vec::new();
                let md_top = rng.gen_range::<usize>(0, 4);
                for _ in 0..md_top {
                    let mut tm = Telemetry_MetadataEntry::new();
                    tm.set_key(rng.choose(&META_KEYS).unwrap().to_string());
                    tm.set_value(rng.choose(&META_VALS).unwrap().to_string());
                    metadata.push(tm);
                }
                if rng.gen::<bool>() {
                    telem.set_persisted(true);
                }
                telem.set_timestamp_ms(time::now() * 1000);
                telem.set_metadata(RepeatedField::from_vec(metadata));
                points.push(telem);
            }

            pyld.set_points(RepeatedField::from_vec(points));

            let pyld_len = pyld.compute_size();
            BigEndian::write_u32(&mut sz_buf, pyld_len);
            bufwrite.write(&sz_buf).unwrap();
            pyld.write_to_writer(&mut bufwrite).unwrap();

            time::sleep_hertz(hertz);
        }
    }
}
