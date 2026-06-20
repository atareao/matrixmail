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

#[cfg(test)]
mod tests {
    use super::*;
    use mail_parser::MessageParser;
    use std::borrow::Cow;

    fn create_test_message() -> Message<'static> {
        let raw_email = b"From: John Doe <john@example.com>\r\n\
            Subject: Test Subject\r\n\
            Message-ID: <abc123@localhost>\r\n\
            Content-Type: text/plain; charset=\"utf-8\"\r\n\
            \r\n\
            Hello, this is the body.";
        MessageParser::new().parse(raw_email).unwrap()
    }

    #[test]
    fn header_mail_new_parses_correctly() {
        let msg = create_test_message();
        let header = HeaderMail::new(42, &msg);
        assert_eq!(header.id, 42);
        assert_eq!(header.reg, "abc123@localhost");
        assert_eq!(header.from, "John Doe <john@example.com>");
        assert_eq!(header.subject, "Test Subject");
    }

    #[test]
    fn mail_new_parses_body() {
        let msg = create_test_message();
        let mail = Mail::new(42, &msg);
        assert_eq!(mail.body, "Hello, this is the body.");
    }

    #[test]
    fn header_mail_display_format() {
        let msg = create_test_message();
        let header = HeaderMail::new(1, &msg);
        let display = format!("{}", header);
        assert!(display.contains("Id: 1"));
        assert!(display.contains("From: John Doe <john@example.com>"));
        assert!(display.contains("Subject: Test Subject"));
    }

    #[test]
    fn mail_display_format() {
        let msg = create_test_message();
        let mail = Mail::new(1, &msg);
        let display = format!("{}", mail);
        assert!(display.contains("Id: 1"));
        assert!(display.contains("Body: Hello, this is the body."));
    }

    #[test]
    fn get_address_none_returns_empty() {
        assert_eq!(HeaderMail::get_address(None), "");
    }

    #[test]
    fn get_from_address_with_name_and_email() {
        let addr = Addr {
            name: Some(Cow::from("Alice")),
            address: Some(Cow::from("alice@example.com")),
        };
        assert_eq!(
            HeaderMail::get_from_address(&addr),
            "Alice <alice@example.com>"
        );
    }

    #[test]
    fn get_from_address_without_name() {
        let addr = Addr {
            name: None,
            address: Some(Cow::from("alice@example.com")),
        };
        assert_eq!(HeaderMail::get_from_address(&addr), " <alice@example.com>");
    }

    #[test]
    fn get_from_address_without_email() {
        let addr = Addr {
            name: Some(Cow::from("Alice")),
            address: None,
        };
        assert_eq!(HeaderMail::get_from_address(&addr), "Alice <>");
    }

    #[test]
    fn get_from_addresses_multiple() {
        let addrs = vec![
            Addr {
                name: Some(Cow::from("Alice")),
                address: Some(Cow::from("alice@example.com")),
            },
            Addr {
                name: Some(Cow::from("Bob")),
                address: Some(Cow::from("bob@example.com")),
            },
        ];
        assert_eq!(
            HeaderMail::get_from_addresses(&addrs),
            "Alice <alice@example.com>, Bob <bob@example.com>"
        );
    }

    #[test]
    fn header_mail_subject_empty_when_missing() {
        let raw_email = b"From: Test <test@example.com>\r\n\
            Message-ID: <no-subject@localhost>\r\n\
            \r\n\
            No subject here";
        let msg = MessageParser::new().parse(raw_email).unwrap();
        let header = HeaderMail::new(1, &msg);
        assert_eq!(header.subject, "");
    }
}
