use std::fmt::Display;
use std::fmt;
use mail_parser::{Message, HeaderValue, Addr, Group};

pub struct Mail{
    id: u32,
    reg: String,
    from: String,
    subject: String,
    body: String,
}

impl Mail {
    pub fn new(id: u32, content: &Message) -> Self{
        let reg = content.message_id().unwrap().to_string();
        let from = Self::get_address(&content.from());
        let subject = content.subject().unwrap_or("").to_string();
        let body = Self::get_body(content);
        Self{
            id,
            reg,
            from,
            subject,
            body,
        }
    }
    fn get_body(content: &Message) -> String{
        let mut s = String::new();
        for part in content.text_bodies(){
            if part.is_text() && part.text_contents().is_some(){
                s.push_str(part.text_contents().unwrap())
            }
        }
        s
    }
    fn get_from_groups(groups: &Vec<Group>) -> String{
        let mut result = Vec::new();
        for group in groups{
            result.push(Self::get_from_addresses(&group.addresses));
        }
        result.join(", ")
    }

    fn get_from_addresses(addresses: &Vec<Addr>) -> String{
        let mut result = Vec::new();
        for address in addresses{
            result.push(Self::get_from_address(address))
        }
        result.join(", ")
    }
    fn get_from_address(address: &Addr) -> String{
        let name = match &address.name{
            Some(name) => name.to_string(),
            None => "".to_string(),
        };
        let mail = match &address.address{
            Some(mail) => mail.to_string(),
            None => "".to_string(),
        };
        format!("{} <{}>", name, mail)
    }

    fn get_address(header: &HeaderValue) -> String{
        match header {
            HeaderValue::Address(address) => Self::get_from_address(address),
            HeaderValue::AddressList(addresses) => Self::get_from_addresses(addresses),
            HeaderValue::Text(text) => text.to_string(),
            HeaderValue::TextList(textlist) => textlist.join(", "),
            HeaderValue::Group(group) => Self::get_from_addresses(&group.addresses),
            HeaderValue::GroupList(groups) => Self::get_from_groups(groups),
            _ => "".to_string(),
        }
    }

}

impl Display for Mail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id: {}\nReg: {}\nFrom: {}\nSubject: {}\nBody: {}",
            self.id, self.reg, self.from, self.subject, self.body)
    }
}

