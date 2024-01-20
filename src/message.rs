use std::path::Display;

pub struct Message {
    shrug: u8,
    status: Status,
    channel: u8,
    note: u8,
    velocity: u8,
    
    raw: [ u8; 256 ],
}

enum Status {
    NoteOn = 144,
    NoteOff = 128,
    Unknown = 0,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Status::NoteOn => "NoteOn".to_string(),
            Status::NoteOff => "NoteOff".to_string(),
            Status::Unknown => "Unknown".to_string(),
        };
        write!(f, "{}", output)
    }
}

// TODO maybe do struct with frequency
// TODO loook up freq, to be more accurate
// i actually dont need an enum
enum Note {

}

// Implement a constructor for Message
impl Message {
    pub fn new(data: [u8; 256]) -> Self {
        let status = match data[1]&0b11110000 {
            144 => Status::NoteOn,
            128 => Status::NoteOff,
            _ => Status::Unknown,
        };

        Message { shrug: data[0], status, channel: data[1]&0b00001111, note: data[2], velocity: data[3], raw: data, }
    }
}

// Implement methods for Message (optional)
impl std::fmt::Display for Message {
    // Example method: Display the message details
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {0}; channel: {1}; velocity: {2}", self.status, self.channel, self.velocity)
    }
}
