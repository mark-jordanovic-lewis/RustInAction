use clap::{App, Arg};
use smoltcp::phy::TapInterface;
use url::Url;

mod dns;
mod ethernet;
mod http;

fn main() {
    let app = App::new("mget")
        .about("GET a web page, manually")
        .arg(Arg::with_name("url").required(true)) // URL to download from
        .arg(Arg::with_name("tap-device").required(true)) // TAP network device to connect to
        .arg(Arg::with_name("dns-server").default_value("1.1.1.1")) // Allows DNS server selection
        .get_matches(); // parse cmd line args

    // get cmd line args
    let url_text = app.value_of("url").unwrap();
    let dns_server_text = app.value_of("dns-server").unwrap();
    let tap_text = app.value_of("tap-device").unwrap();
    // validate url arg
    let url = Url::parse(url_text).expect("error: unable to parse <url> as a URL");
    if url.scheme != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }
    // validate tap arg
    let tap = TapInterface::new(&tap_text)
        .expect("error: unable to use <tap-device> as a network interface");
    // validate dns arg
    let domain_name = url.host().expect("domain name required");
    let _dns_server: std::net::Ipv4Addr = dns_server_text
        .parse()
        .expect("error: unable to parse <dns-server> as an IPv4 address");
    // get ip addr of fqdn
    let addr = dns::resolve(dns_server_text, &domain_name.to_string())
        .unwrap()
        .unwrap();
    // generate random MAC addr
    let mac = ethernet::MacAddress::new().into();

    // make the request
    http::get(tap, mac, addr, url).unwrap();
}
