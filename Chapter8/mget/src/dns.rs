use std::error::Error;
use std::fmt::Formatter;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::proto::error::ProtoError;
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn message_id() -> u16 {
    let candidate = rand::random();
    if candidate == 0 {
        return message_id();
    }
    candidate
}

#[derive(Debug)]
pub enum DnsError {
    ParseDomainName(ProtoError),
    ParseDnsServerAddress(std::net::AddrParseError),
    Encoding(ProtoError),
    Decoding(ProtoError),
    Network(std::io::Error),
    Sending(std::io::Error),
    Receiving(std::io::Error),
}

impl std::fmt::Display for DnsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DnsError {}

pub fn resolve(
    dns_server_address: &str,
    domain_name: &str,
) -> Result<Option<std::net::IpAddr>, Box<dyn Error>> {
    let domain_name = Name::from_ascii(domain_name).map_err(DnsError::ParseDomainName)?;
    let dns_server_address = format!("{}:53", dns_server_address);
    // build SocketAddress from raw text
    let dns_server: SocketAddr = dns_server_address
        .parse()
        .map_err(DnsError::ParseDnsServerAddress)?;
    // only need a small buffer for the dns req
    let mut request_buffer: Vec<u8> = Vec::with_capacity(64);
    // DNS over UDP has max packet size of 512 bytes
    let mut response_buffer: Vec<u8> = vec![0; 512];

    // We only use a single DNS query, but can be multiple
    let mut request = Message::new();
    request.add_query(Query::query(domain_name, RecordType::A));
    request
        .set_id(message_id())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true); // Allow DNS to make additional req's if it doesn't hold ip

    // bind to port 0 allow OS to choose port to bind to
    let localhost = UdpSocket::bind("0.0.0.0:0").map_err(DnsError::Network)?;
    let timeout = Duration::from_secs(5);
    localhost
        .set_read_timeout(Some(timeout))
        .map_err(DnsError::Network)?;
    localhost
        .set_nonblocking(false)
        .map_err(DnsError::Network)?;

    let mut encoder = BinEncoder::new(&mut request_buffer);
    request.emit(&mut encoder).map_err(DnsError::Encoding)?;

    let _n_bytes_sent = localhost
        .send_to(&request_buffer, dns_server)
        .map_err(DnsError::Network)?;

    loop {
        // ignore other, unexpected, DNS messages sent to 'our' port
        let (_b_bytes_received, remote_port) = localhost
            .recv_from(&mut response_buffer)
            .map_err(DnsError::Receiving)?;
        // when expected message comes, continue processing
        if remote_port == dns_server {
            break;
        }
    }

    let response = Message::from_vec(&response_buffer).map_err(DnsError::Decoding)?;

    for answer in response.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let server_ip = resource.to_ip_addr().expect("invalid IP address received");
            return Ok(Some(server_ip));
        }
    }
    Ok(None)
}
