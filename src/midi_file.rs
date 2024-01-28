use std::io::Read;
use std::thread::sleep;
use std::time::SystemTime;

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
        
        println!("{:?} {} {}; ticks per quater note: {} ({:x?})", header.length, header.format, header.tracks, header.division, header.division);
        let data = buffer[22..(track.length+22) as usize].to_vec();
        println!("{:?}", track.length);
        println!("{:X?}", data);

        // loops over each event
        let mut i = 0; // want this in a for, but i will barely use the i++
        while i < data.len() {
            // let delta_time = data[i];
            // TODO maybe length is at least 2B
            let mut delta_time = (data[i]&0b0111_1111) as u64;
            println!("\n\tdata: {:X?}; {}; hex: 0x{:X?}",data[i], data[i]&0b0111_1111, data[i]&0b0111_1111);
            // println!("\n\tdelta: {}; hex: 0x{:X?}", delta_time, delta_time);
            // keep looping until delta time most sig bit is 
            while data[i]&0b1000_0000 != 0 {
                // println!("############################ 0x{:X?}", data[i]&0b1000_0000);
                i+=1;
                println!("\t{:X?} {}", data[i], data[i]);
                // we must ignore first bit
                delta_time = delta_time << 7 | (data[i]&0b0111_1111) as u64;
            }
            i+=1;

            println!("i: {}; delta: {}", i, delta_time);
            println!("i: {}; delta: {}; code: 0x{:X?}", i, delta_time, data[i]);
            // how long is a beat
            // sleep(delta_time);

            match data[i]&0b1111_0000 {
                0xF0 => {
                    let code = as_u16_be(data[i..i+2].try_into().expect("length failed"));
                    i+=2;

                    print!("\tformated: code: 0x{:X?}; data: ", code);
                    match code&0b0000_0000_1111_1111 {
                        0x0001 | 0x0002 | 0x0003 | 0x0004 | 0x0005 | 0x0006 | 0x0007 | 0x0008 | 0x0009 | 0x000A | 0x000B | 0x000C | 0x000D | 0x000E | 0x000CF  => {
                            let mut length: usize = (data[i]&0b0111_1111) as usize;
                            while data[i]&0b1000_0000 != 0 {
                                // println!("############################ 0x{:X?}", data[i]&0b1000_0000);
                                i+=1;
                                // we must ignore first bit
                                length = length << 7 | (data[i]&0b0111_1111) as usize;
                            }
                            i+=length+1;

                            print!("length: {}; ", length);
                            println!("{:?}", std::str::from_utf8(&data[i-length..i]));
                        }
                        0x0058 => {
                            let length: usize = data[i] as usize;
                            i+=1;
                            println!("{}/{}; clocks per metronome tick: {}; 32nd notes per quater note: {}", data[i], 2_u8.pow(data[i+1] as u32), data[i+2], data[i+3]);
                            i+=4;
                        },
                        0x0059 => {
                            let length: usize = data[i] as usize;
                            i+=1;
                            println!("{} {}; key: {}", (data[i] as i8).abs(), 
                            if (data[i] as i8) < 0 {"flats"} else {"sharps"}, 
                            if data[i+1] == 0 {"major key"} else {"minor key"});
                            i+=2;
                        }
                        0x0051 => {
                            let length: usize = data[i] as usize;
                            i+=1;
                            println!("{} μs/quarter-note", as_u24_be(data[i..i+3].try_into().expect("length failed")));
                            i+=3;
                        }
                        _ => {
                            println!("unknown 0x{:X?}", code&0b0000_0000_1111_1111);
                            break;
                        }
                    }
                }
                0xC0 => {
                    println!("channel: {}; controller: {}; value: {}", data[i]&0x0F, data[i+1], data[i+2]);
                    i+=3;
                    // break;
                }
                0xB0 => {
                    println!("Control Change: channel: {}; controller: {}; value: {}", data[i]&0b0000_1111, data[i+1], data[i+2]);
                    i+=3;
                }
                0x80 => {
                    println!("0x{:X?} 0x{:X?} 0x{:X?}", data[i], data[i+1], data[i+2]);
                    println!("{}", Message::from_midi(data[i], data[i+1], data[i+2]));
                    i+=3;
                }
                0x90 => {
                    println!("0x{:X?} 0x{:X?} 0x{:X?}", data[i], data[i+1], data[i+2]);
                    println!("{}", Message::from_midi(data[i], data[i+1], data[i+2]));
                    i+=3;
                }
                0x70 => {
                    match data[i]&0b0000_1111 {
                        0x01|0x02|0x03|0x04|0x05|0x06|0x07 => {
                            i+=1;
                            println!("undefined: {}", data[i]);
                            i+=1;
                        }
                        0x09 => {
                            i+=1;
                            println!("Reset All Controllers: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown 0x{:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x60 => {
                    match data[i]&0b0000_1111 {
                        0x03 => {
                            i+=1;
                            println!("Non-Registered Parameter Number MSB: {}", data[i]);
                            i+=1;
                        }
                        0x04 => {
                            i+=1;
                            println!("Registered Parameter Number LSB: {}", data[i]);
                            i+=1;
                        }
                        0x06|0x07|0x08|0x09|0x0A|0x0B|0x0C|0x0D|0x0E|0x0F => {
                            i+=1;
                            println!("undefined: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown 0x{:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x50 => {
                    match data[i]&0b0000_1111 {
                        0x00 => {
                            i+=1;
                            println!("General Purpose Controller #5: {}", data[i]);
                            i+=1;
                        }
                        0x04 => {
                            i+=1;
                            println!("Portamento Control: {}", data[i]);
                            i+=1;
                        }
                        0x0B => {
                            i+=1;
                            println!("effects 1 depth: {}", data[i]);
                            i+=1;
                        }
                        0x0D => {
                            i+=1;
                            println!("effects 3 depth: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown 0x{:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x40 => {
                    match data[i]&0b0000_1111 {
                        0x00 => {
                            i+=1;
                            println!("Damper pedal on/off (Sustain): {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x01 => {
                            i+=1;
                            println!("Portamento on/off: {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x02 => {
                            i+=1;
                            println!("Sustenuto: {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x03 => {
                            i+=1;
                            println!("Soft pedal on/off: {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x04 => {
                            i+=1;
                            println!("Legato Footswitch: {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x05 => {
                            i+=1;
                            println!("Hold 2: {}", if data[i] < 63 {"off"} else {"on"});
                            i+=1;
                        }
                        0x06 => {
                            i+=1;
                            println!("Sound Controller 1 (Sound Variation): {}", data[i]);
                            i+=1;
                        }
                        0x07 => {
                            i+=1;
                            println!("Sound Controller 2 (Timbre): {}", data[i]);
                            i+=1;
                        }
                        0x08 => {
                            i+=1;
                            println!("Sound Controller 3 (shrug): {}", data[i]);
                            i+=1;
                        }
                        0x09 => {
                            i+=1;
                            println!("Sound Controller 4 (Attack Time): {}", data[i]);
                            i+=1;
                        }
                        0x0A => {
                            i+=1;
                            println!("Sound Controller 5 (Brightness): {}", data[i]);
                            i+=1;
                        }
                        0x0B => {
                            i+=1;
                            println!("Sound Controller 6: {}", data[i]);
                            i+=1;
                        }
                        0x0C => {
                            i+=1;
                            println!("Sound Controller 7: {}", data[i]);
                            i+=1;
                        }
                        0x0D => {
                            i+=1;
                            println!("Sound Controller 8: {}", data[i]);
                            i+=1;
                        }
                        0x0E => {
                            i+=1;
                            println!("Sound Controller 9: {}", data[i]);
                            i+=1;
                        }
                        0x0F => {
                            i+=1;
                            println!("Sound Controller 10: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown {:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x30 => {
                    match data[i]&0b0000_1111 {
                        0x04|0x05|0x06|0x07|0x08|0x09|0x0A|0x0B|0x0C|0x0D|0x0E|0x0F => {
                            i+=1;
                            println!("undefined: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown {:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x10 => {
                    match data[i]&0b0000_1111 {
                        0x04|0x05|0x06|0x07|0x08|0x09|0x0A|0x0B|0x0C|0x0D|0x0E|0x0F => {
                            i+=1;
                            println!("undefined: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown {:X?}", data[i]);
                            break;
                        }
                    }
                }
                0x00 => {
                    match data[i]&0b0000_1111 {
                        0x07 => {
                            i+=1;
                            println!("channel volume: {}", data[i]);
                            i+=1;
                        }
                        0x0A => {
                            i+=1;
                            println!("pan: {}", data[i]);
                            i+=1;
                        }
                        0x01 => {
                            i+=1;
                            println!("Modulation wheel: {}", data[i]);
                            i+=1;
                        }
                        0x00 => {
                            i+=1;
                            println!("Bank Select: {}", data[i]);
                            i+=1;
                        }
                        _ => {
                            println!("unknown {:X?}", data[i]);
                            break;
                        }
                    }
                }
                _ => {
                    println!("unknown {:X?}", data[i]);
                    break;
                }
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
