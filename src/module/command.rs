// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

#[derive(Debug, PartialEq)]
enum Type {
    Set,
    Get,
    Update,
    Delete,
    Ping,
    Exit,
    Unknown,
}

#[derive(Debug)]
struct Command {
    key: String,
    value: String,
    expire: i64,
    name: Type,
}

impl Command {
    pub fn new() -> Command {
        Command {
            key: "".to_string(),
            value: "".to_string(),
            expire: 0,
            name: Type::Unknown,
        }
    }

    pub fn from_str(cmd: String) -> Result<Command, String> {
        let name_val: Type;

        let command = cmd.trim().to_string();
        let mut items: Vec<&str> = command.split(' ').collect();

        // Match the command
        match items[0] {
            "SET" => {
                name_val = Type::Set;
            }
            "GET" => {
                name_val = Type::Get;
            }
            "UPDATE" => {
                name_val = Type::Update;
            }
            "DELETE" => {
                name_val = Type::Delete;
            }
            "PING" => {
                name_val = Type::Ping;
            }
            "EXIT" => {
                name_val = Type::Exit;
            }
            _ => {
                name_val = Type::Unknown;
            }
        }

        // If an invalid command, raise an error
        if name_val == Type::Unknown {
            return Err(format!(
                "invalid command name_val {name_val}",
                name_val = items[0]
            ));
        }

        if (items.len() < 2)
            && ((name_val == Type::Get)
                || (name_val == Type::Delete)
                || (items[1] == ""))
        {
            return Err(format!("invalid command {cmd}", cmd = cmd));
        }

        if (items.len() < 3)
            && ((name_val == Type::Set)
                || (name_val == Type::Update)
                || (items[1] == "")
                || (items[2] == ""))
        {
            return Err(format!("invalid command {cmd}", cmd = cmd));
        }

        while items.len() < 4 {
            items.push("");
        }

        if items[3] == "" {
            items[3] = "0"
        }

        Ok(Command {
            key: items[1].to_string(),
            value: items[2].to_string(),
            expire: items[3].to_string().parse::<i64>().unwrap(),
            name: name_val,
        })
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_expire(&self) -> &i64 {
        &self.expire
    }

    pub fn get_name(&self) -> &Type {
        &self.name
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value
    }

    pub fn set_expire(&mut self, expire: i64) {
        self.expire = expire
    }

    pub fn set_name(&mut self, name: Type) {
        self.name = name
    }
}

#[test]
fn test_command() {
    let mut cmd: Command;

    cmd = Command::new();

    cmd.set_key("item1".to_string());
    cmd.set_value("value1".to_string());
    cmd.set_expire(0);
    cmd.set_name(Type::Set);

    assert_eq!(*cmd.get_key(), "item1".to_string());
    assert_eq!(*cmd.get_value(), "value1".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Set);

    // Test `SET $key $value $expire` command
    match Command::from_str("SET item2 value2".to_string()) {
        Ok(v) => {
            cmd = v;
        }
        Err(_) => {}
    }

    assert_eq!(*cmd.get_key(), "item2".to_string());
    assert_eq!(*cmd.get_value(), "value2".to_string());
    assert_eq!(*cmd.get_expire(), 0);
    assert_eq!(*cmd.get_name(), Type::Set);
}
