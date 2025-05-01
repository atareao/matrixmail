use mail_parser::{Addr, Address, Group, Message};
use std::fmt;
use std::fmt::Display;

pub struct HeaderMail {
    id: u32,
    reg: String,
    from: String,
    subject: String,
}
pub struct Mail {
    header: HeaderMail,
    body: String,
}

impl Mail {
    pub fn new(id: u32, content: &Message) -> Self {
        let header = HeaderMail::new(id, content);
        let body = Self::get_body(content);
        Self { header, body }
    }

    fn get_body(content: &Message) -> String {
        let mut s = String::new();
        for part in content.text_bodies() {
            if part.is_text() && part.text_contents().is_some() {
                s.push_str(part.text_contents().unwrap())
            }
        }
        s
    }
}

impl HeaderMail {
    pub fn new(id: u32, content: &Message) -> Self {
        let reg = content.message_id().unwrap().to_string();
        let from = Self::get_address(content.from());
        let subject = content.subject().unwrap_or("").to_string();
        Self {
            id,
            reg,
            from,
            subject,
        }
    }
    fn get_from_groups(groups: &Vec<Group>) -> String {
        let mut result = Vec::new();
        for group in groups {
            result.push(Self::get_from_addresses(&group.addresses));
        }
        result.join(", ")
    }

    fn get_from_addresses(addresses: &Vec<Addr>) -> String {
        let mut result = Vec::new();
        for address in addresses {
            result.push(Self::get_from_address(address))
        }
        result.join(", ")
    }
    fn get_from_address(address: &Addr) -> String {
        let name = match &address.name {
            Some(name) => name.to_string(),
            None => "".to_string(),
        };
        let mail = match &address.address {
            Some(mail) => mail.to_string(),
            None => "".to_string(),
        };
        format!("{} <{}>", name, mail)
    }

    fn get_address(address: Option<&Address>) -> String {
        match address {
            Some(address) => match address {
                Address::List(addresses) => Self::get_from_addresses(addresses),
                Address::Group(groups) => Self::get_from_groups(groups),
            },
            None => "".to_string(),
        }
    }
}

impl Display for HeaderMail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {}\nReg: {}\nFrom: {}\nSubject: {}",
            self.id, self.reg, self.from, self.subject
        )
    }
}

impl Display for Mail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {}\nReg: {}\nFrom: {}\nSubject: {}\nBody: {}",
            self.header.id, self.header.reg, self.header.from, self.header.subject, self.body
        )
    }
}
