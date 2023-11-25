mod PacketModule;

use PacketModule::GamePacket::ChatMessagePacket;
use PacketModule::GamePacket::Serializable;

fn main() {

    let chat_packet = ChatMessagePacket::new("HELLO PACKET TEST");


    let serialized_data = chat_packet.serialize();
    println!("serialize: {}", serialized_data);


    let mut deserialized_packet = ChatMessagePacket::new("");
    deserialized_packet.deserialize(&serialized_data);
    println!("deserialize: {}", deserialized_packet.get_message());
}
