use core::time;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crate::message::{self, Message};

pub struct MidiFile {
    filename: String,
    header: MidiHeader,
    tracks: Vec<MidiTrack>,
}

struct MidiHeader {
    length: u32,
    format: u16,
    n_tracks: u16,
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
            n_tracks: as_u16_be(buffer[10..12].try_into().expect("tracks failed")),
            division: as_u16_be(buffer[12..14].try_into().expect("division failed")),
        };

        println!("{:?} {} {}; ticks per quater note: {} ({:x?})", header.length, header.format, header.n_tracks, header.division, header.division);

        let mut messages: Vec<Message> = Vec::new();

        let mut tracks = Vec::new();
        let mut time_per_qn = 0;
        let mut j = 14;
        while j < buffer.len() {
            println!("{} {} {} {}", buffer[j] as char, buffer[j+1] as char, buffer[j+2] as char, buffer[j+3] as char);
            j+=4;

            let mut track = MidiTrack {
                length: as_u32_be(buffer[j..j+4].try_into().expect("length failed")),
                events: Vec::new(),
            };
            j+=4;
            
            let data = buffer[j..(track.length+j as u32) as usize].to_vec();
            println!("{:?}", track.length);
            println!("{:X?}", data);
    
            // loops over each event
            let mut running_status = false;
            let mut code_channel = 0;
            let mut i = 0; // want this in a for, but i will barely use the i++
            let mut current_time = 0;
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
                current_time += delta_time * (time_per_qn / header.division as u64) / 1000;
                println!("\tcurrent_time: {}", current_time);
                
    
                println!("i: {}; delta: {}", i, delta_time);
                println!("i: {}; delta: {}; code: 0x{:X?}", i, delta_time, data[i]);
                // how long is a beat
                // sleep(delta_time);
    
                // TODO this does not seem right, must be jumping over something
                if !running_status || (data[i] == 0xFF && data[i+1] == 0x2F) {
                    println!("hi {} {}", running_status, i+j);
                    code_channel = data[i];
                }
                println!("code_channel: 0x{:X?} (0x{:X?})", code_channel, data[i]);
    
                match code_channel&0b1111_0000 {
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
                                time_per_qn = as_u24_be(data[i..i+3].try_into().expect("length failed"));
                                println!("{} μs/quarter-note", time_per_qn);
                                i+=3;

                            }
                            0x002F => {
                                i+=1;
                                running_status = false;
                                println!("end of track");
                                i+=1;
                                break;
                            }
                            0x0021 => {
                                i+=1;
                                println!("shrug {:X?} {:X?} {:X?} {:X?}", data[i], data[i+1], data[i+2], data[i+3]);
                                i+=1;
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
                    0x90 | 0x80 => {
                        // if !running_status {
                        //     code_channel = data[i];
                        // }
                        println!("data: i: 0x{:X?}; i+1: 0x{:X?}; i+2: 0x{:X?}", data[i], data[i+1], data[i+2]);
    
                        let number = if running_status {
                            data[i]
                        } else {
                            data[i+1]
                        };
    
                        let velocity = if running_status {
                            data[i+1]
                        } else {
                            data[i+2]
                        };
    
                        // if the velocity is 0, set the code part to 0x8?
                        code_channel = if velocity == 0 {
                            code_channel&0b0000_1111|0x80
                        } else {
                            code_channel&0b0000_1111|0x90
                        };
                        println!("code channe: 0x{:X?}; number: 0x{:X?}; velocity: 0x{:X?}", code_channel, number, velocity);
    
                        println!("{}", Message::from_midi(code_channel, number, velocity, Duration::from_millis(current_time)));
                        let event = MidiEvent{delta_time: delta_time as u32, event: Message::from_midi(code_channel, number, velocity, Duration::from_millis(current_time))};
                        track.events.push(event);
                        messages.push(Message::from_midi(code_channel, number, velocity, Duration::from_millis(current_time)));
                        
                        if running_status {
                            i+=2;
                        } else {
                            i+=3;
                            running_status = true;
                        }
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
                                // i+=1;
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

            tracks.push(track);
            j += i-1;
        }

        messages.sort_by(|a, b| a.cmp(b));
        let time_start = SystemTime::now();
        let mut current_message = 0;
        println!("{:?}", messages.len());
        sleep(Duration::from_millis(2000));
        loop {
            if time_start.elapsed().unwrap() >= messages[current_message].play_at {
                println!("{:}", messages[current_message]);
                current_message += 1;
            }
            if current_message >= messages.len() {
                break;
            }
        }

        Self { filename: "".to_owned(), header, tracks }
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
