use std::fmt;


#[derive(Debug)]
enum PacketType {
    ChatMessage = 1,
    PlayerPosition,
}

pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(&mut self, data: &str);
}

pub trait Packet: Serializable {
    fn get_type(&self) -> PacketType;
}

#[derive(Debug)]
pub struct ChatMessagePacket {
    message: String,
}

impl ChatMessagePacket {
    pub fn new(message: &str) -> Self {
        ChatMessagePacket {
            message: message.to_string(),
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl Packet for ChatMessagePacket {
    fn get_type(&self) -> PacketType {
        PacketType::ChatMessage
    }
}

impl Serializable for ChatMessagePacket {
    fn serialize(&self) -> String {
        format!("{}|{}", self.get_type() as u8, self.message)
    }

    fn deserialize(&mut self, data: &str) {
        let parts: Vec<&str> = data.split('|').collect();
        if let Some(msg) = parts.get(1) {
            self.message = msg.to_string();
        }
    }
}
