#![allow(unused_variables, dead_code)]

use chrono::offset::Utc;

#[derive(Debug, PartialEq, Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[derive(Debug, Clone)]
struct Message {
    to: u64,
    message: String,
    id: u64
}

impl Message {
    fn new(to: u64, message: String, id: u64) -> Self {
        Message { to, message, id }
    }
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
    sent_count: u64
}

impl MailBox {
    fn new() -> Self {
        MailBox {
            messages: vec![],
            sent_count: 0
        }
    }

    fn post(&mut self, to: u64, msg: String) {
        let message = Message::new(to, msg, self.sent_count);
        self.messages.push(message);
        self.sent_count += 1
    }

    fn deliver(&mut self, cubesat_id: u64) -> Vec<Message> {
        let (mut response, mut held_messages): (Vec<Message>, Vec<Message>) = (vec![], vec![]);
        self.messages.iter().for_each(|message|
            if message.to == cubesat_id {
                response.push(message.clone())
            } else {
                held_messages.push(message.clone())
            }
        );
        self.messages = held_messages;
        response
    }

    fn store_bulk(&mut self, messages: &mut Vec<Message>) {
        self.messages.append(messages);
    }

    fn len(&self) -> usize {
       self.messages.len()
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
    status: StatusMessage
}

// write ping pong implementation between satellites
impl CubeSat {
    fn new(id: u64) -> Self {
        CubeSat {
            id,
            mailbox: MailBox::new(),
            status: StatusMessage::Ok
        }
    }

    fn check_status(&mut self) -> StatusMessage {
        self.set_status();
        println!(
            "Status of Satellite #{}: {:?}. Message count: {}",
            self.id, self.status, self.unread_messages()
        );
        self.status
    }

    fn set_status(&mut self) {
        self.status = StatusMessage::Ok
    }

    fn recv(&mut self, mailbox: &mut MailBox) {
        self.mailbox.messages = mailbox.deliver(self.id);
        println!("{} Messages held by satellite #{}", self.mailbox.len(), self.id)
    }

    fn print_messages(&mut self) {
        while self.mailbox.messages.len() > 0 {
            let message = self.mailbox.messages.pop().unwrap();
            println!("message#{}: {}", message.id, message.message);
        }
    }

    fn unread_messages(&self) -> usize {
        self.mailbox.len()
    }
}

struct GroundStation {
    mailbox: MailBox,
    live_satellites: Vec<CubeSat>
}

impl GroundStation {
    fn new() -> GroundStation {
        GroundStation {
            mailbox: MailBox::new(),
            live_satellites: vec![]
        }
    }

    fn connect(&mut self, sat_id: u64) -> u64 {
        // check server for live satellites connected to this GroundStation
        // pull in or build fresh.
        // Should hold a connection details for the ServerStation and
        // id of the satellite rather than the satellite itself.
        let new_satellite = CubeSat::new(sat_id);
        self.live_satellites.push(new_satellite);
        sat_id // better to generate this on creation (*)
    }

    fn send(&mut self, to: u64, msg: String) {
        self.mailbox.post(to, msg);
    }
}

fn main() {
    let mut base = GroundStation::new();
    let satellite_ids = fetch_satellite_ids();
    let mut live_satellite_ids: Vec<u64> = vec![];

    for id in satellite_ids {
        live_satellite_ids.push(base.connect(id)); // (*) and save to file here for reconnection to server.
    }

    for id in live_satellite_ids {
        base.send(id, format!("Initialised Connection. {}", Utc::now()))
    }

    for mut cubesat in base.live_satellites {
        cubesat.recv(&mut base.mailbox);
        cubesat.print_messages();
    }
}

fn fetch_satellite_ids() -> Vec<u64> {
    vec![1,2,3]
}