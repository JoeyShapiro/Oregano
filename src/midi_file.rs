use std::io::Read;

use crate::message::Message;

pub struct MidiFile {
    filename: String,
    // header: MidiHeader,
}

struct MidiHeader {
    length: u32,
    format: u16,
    tracks: u16,
    division: u16,
}

struct MidiTrack {
    length: u32,
    events: Vec<MidiEvent>,
}

struct MidiEvent {
    delta_time: u32,
    event: Message,
}

impl MidiFile {
    pub fn new(filename: String) -> Self {
        // open the file
        let mut file = std::fs::File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let header = MidiHeader {
            length: as_u32_be(buffer[4..8].try_into().expect("length failed")),
            format: as_u16_be(buffer[8..10].try_into().expect("format failed")),
            tracks: as_u16_be(buffer[10..12].try_into().expect("tracks failed")),
            division: as_u16_be(buffer[12..14].try_into().expect("division failed"))
        };

        println!("{} {} {} {}", buffer[14] as char, buffer[15] as char, buffer[16] as char, buffer[17] as char);

        let track = MidiTrack {
            length: as_u32_be(buffer[18..22].try_into().expect("length failed")),
            events: Vec::new(),
        };
        
        println!("{:?} {} {} {}", header.length, header.format, header.tracks, header.division);
        let test = buffer[22..track.length as usize].to_vec();
        println!("{:?}", track.length);

        // i need a classic for loop to loop over it

        let mut data: Vec<u8> = Vec::new();
        // code - 2B
        // length - 1B
        // stuff - length B
        // FF is meta
        // 00 is escape
        // TODO thats what they mean by var length
        // some are only a few bytes, depending on what the status code needs
        let mut length = 0_u8;
        for b in test {
            if data.len() == 0 && b == 0x00 {
                continue;
            } else if data.len() == 1 {
                data.push(b);
            } else if data.len() == 2 {
                data.push(b);
                length = b;
            } else {
                data.push(b);
            }

            if data.len() == (length+3).into() {
                println!("raw ({}): {:?}", data.len(), data);

                let code: u16 = as_u16_be(data[0..2].try_into().expect("length failed"));

                print!("\tformated: code: {:X?}; data: ", code);
                match code&0b1111_1111_0000_0000 {
                    0xFF00 => {
                        let length: u8 = data[2];
                        let offset: usize = 2+length as usize;

                        print!("length: {}; ", length);
                        match code&0b0000_0000_1111_1111 {
                            0x0008 => println!("{:?}", std::str::from_utf8(&data[3..offset])),
                            0x0009 => println!("{:?}", std::str::from_utf8(&data[3..offset])),
                            0x000C => println!("{:?}", std::str::from_utf8(&data[3..offset])),
                            0x0058 => println!("{}/{}; clocks per metronome tick: {}; 32nd notes per quater note: {}", data[3], 2_u8.pow(data[4] as u32), data[5], data[6]),
                            0x0059 => println!("{} {}; key: {}", (data[3] as i8).abs(), 
                                if (data[3] as i8) < 0 {"flats"} else {"sharps"}, 
                                if data[4] == 0 {"major key"} else {"minor key"}),
                            0x0051 => println!("{} ms/quarter-note", as_u24_be(data[3..6].try_into().expect("length failed"))),
                            _ => {
                                println!("unknown {:X?}", code&0b0000_0000_1111_1111);
                                break;
                            }
                        }
                    },
                    0xC000 => println!("channel: {}; controller: {}; value: {}", data[0]&0x0F, data[1], data[2]),
                    _ => {
                        println!("unknown {:X?}", code&0b1111_1111_0000_0000);
                        break;
                    },
                };

                data.clear();
            }
        }

        Self { filename: "".to_owned() }
    }
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u16_be(array: &[u8; 2]) -> u16 {
    ((array[0] as u16) <<  8) +
    ((array[1] as u16) <<  0)
}

// why
fn as_u24_be(array: &[u8; 3]) -> u64 {
    ((array[0] as u64) << 16) +
    ((array[1] as u64) <<  8) +
    ((array[2] as u64) <<  0)
}
