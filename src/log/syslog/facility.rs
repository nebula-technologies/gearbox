pub enum Facility {
    Kernel,
    User,
    Mail,
    System,
    Security,
    Syslog,
    Printer,
    News,
    Uucp,
    Clock,
    Auth,
    Ftp,
    Ntp,
    Audit,
    Alert,
    Clock2,
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}

impl Facility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Facility::Kernel => "kernel",
            Facility::User => "user",
            Facility::Mail => "mail",
            Facility::System => "system",
            Facility::Security => "security",
            Facility::Syslog => "syslog",
            Facility::Printer => "printer",
            Facility::News => "news",
            Facility::Uucp => "uucp",
            Facility::Clock => "clock",
            Facility::Auth => "auth",
            Facility::Ftp => "ftp",
            Facility::Ntp => "ntp",
            Facility::Audit => "audit",
            Facility::Alert => "alert",
            Facility::Clock2 => "clock2",
            Facility::Local0 => "local0",
            Facility::Local1 => "local1",
            Facility::Local2 => "local2",
            Facility::Local3 => "local3",
            Facility::Local4 => "local4",
            Facility::Local5 => "local5",
            Facility::Local6 => "local6",
            Facility::Local7 => "local7",
        }
    }

    pub fn as_int(&self) -> u8 {
        match self {
            Facility::Kernel => 0,
            Facility::User => 1,
            Facility::Mail => 2,
            Facility::System => 3,
            Facility::Security => 4,
            Facility::Syslog => 5,
            Facility::Printer => 6,
            Facility::News => 7,
            Facility::Uucp => 8,
            Facility::Clock => 9,
            Facility::Auth => 10,
            Facility::Ftp => 11,
            Facility::Ntp => 12,
            Facility::Audit => 13,
            Facility::Alert => 14,
            Facility::Clock2 => 15,
            Facility::Local0 => 16,
            Facility::Local1 => 17,
            Facility::Local2 => 18,
            Facility::Local3 => 19,
            Facility::Local4 => 20,
            Facility::Local5 => 21,
            Facility::Local6 => 22,
            Facility::Local7 => 23,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facility_as_str() {
        assert_eq!(Facility::Kernel.as_str(), "kernel");
        assert_eq!(Facility::User.as_str(), "user");
        assert_eq!(Facility::Mail.as_str(), "mail");
        assert_eq!(Facility::System.as_str(), "system");
        assert_eq!(Facility::Security.as_str(), "security");
        assert_eq!(Facility::Syslog.as_str(), "syslog");
        assert_eq!(Facility::Printer.as_str(), "printer");
        assert_eq!(Facility::News.as_str(), "news");
        assert_eq!(Facility::Uucp.as_str(), "uucp");
        assert_eq!(Facility::Clock.as_str(), "clock");
        assert_eq!(Facility::Auth.as_str(), "auth");
        assert_eq!(Facility::Ftp.as_str(), "ftp");
        assert_eq!(Facility::Ntp.as_str(), "ntp");
        assert_eq!(Facility::Audit.as_str(), "audit");
        assert_eq!(Facility::Alert.as_str(), "alert");
        assert_eq!(Facility::Clock2.as_str(), "clock2");
        assert_eq!(Facility::Local0.as_str(), "local0");
        assert_eq!(Facility::Local1.as_str(), "local1");
        assert_eq!(Facility::Local2.as_str(), "local2");
        assert_eq!(Facility::Local3.as_str(), "local3");
        assert_eq!(Facility::Local4.as_str(), "local4");
        assert_eq!(Facility::Local5.as_str(), "local5");
        assert_eq!(Facility::Local6.as_str(), "local6");
        assert_eq!(Facility::Local7.as_str(), "local7");
    }

    #[test]
    fn test_facility_as_int() {
        assert_eq!(Facility::Kernel.as_int(), 0);
        assert_eq!(Facility::User.as_int(), 1);
        assert_eq!(Facility::Mail.as_int(), 2);
        assert_eq!(Facility::System.as_int(), 3);
        assert_eq!(Facility::Security.as_int(), 4);
        assert_eq!(Facility::Syslog.as_int(), 5);
        assert_eq!(Facility::Printer.as_int(), 6);
        assert_eq!(Facility::News.as_int(), 7);
        assert_eq!(Facility::Uucp.as_int(), 8);
        assert_eq!(Facility::Clock.as_int(), 9);
        assert_eq!(Facility::Auth.as_int(), 10);
        assert_eq!(Facility::Ftp.as_int(), 11);
        assert_eq!(Facility::Ntp.as_int(), 12);
        assert_eq!(Facility::Audit.as_int(), 13);
        assert_eq!(Facility::Alert.as_int(), 14);
        assert_eq!(Facility::Clock2.as_int(), 15);
        assert_eq!(Facility::Local0.as_int(), 16);
        assert_eq!(Facility::Local1.as_int(), 17);
        assert_eq!(Facility::Local2.as_int(), 18);
        assert_eq!(Facility::Local3.as_int(), 19);
        assert_eq!(Facility::Local4.as_int(), 20);
        assert_eq!(Facility::Local5.as_int(), 21);
        assert_eq!(Facility::Local6.as_int(), 22);
        assert_eq!(Facility::Local7.as_int(), 23);
    }
}
