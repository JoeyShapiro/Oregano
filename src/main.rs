use core::time;
use std::alloc::System;
use std::env::current_exe;
use std::os::macos::raw::stat;
use std::time::{Duration, SystemTime};
use std::{default, thread};
use std::sync::mpsc;
use eframe::egui;
use std::sync::{Arc, Mutex};

mod message;
use egui::epaint::RectShape;
use message::Message;
mod midi_file;
use midi_file::MidiFile;

use rusb::{
    Context, Device, DeviceDescriptor, DeviceHandle, Direction, Result, TransferType, UsbContext,
};

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn convert_argument(input: &str) -> u16 {
    if input.starts_with("0x") {
        return u16::from_str_radix(input.trim_start_matches("0x"), 16).unwrap();
    }
    u16::from_str_radix(input, 10)
        .expect("Invalid input, be sure to add `0x` for hexadecimal values.")
}

fn main() {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Oregano",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            

            Box::new(MyApp::new(cc)) // Box::<MyApp>::default()
        }),
    );
    panic!("Hello, world!");

    // let sevend: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x7F];
    let sevend: [u8; 2] = [ 0xFF, 0x7F ];
    println!("{:X?}", 0b0111_1111+0b0111_1111);
    println!("{:X?}", 0b111_1111_111_1111);
    // i need to concat
    // and i need the 0s
    
    let mut i = 0;
    let mut out = (sevend[i]&0b0111_1111) as u64;
    println!("\t{:X?} {}", sevend[i], sevend[i]);
    while sevend[i]&0b1000_0000 != 0 {
        i+=1;
        println!("\t{:X?} {}", sevend[i], sevend[i]);
        // we must ignore first bit
        out = out << 7 | (sevend[i]&0b0111_1111) as u64;
    }
    println!("out: {} ({:X?})", out, out);
    let midi = MidiFile::new("Bad_Apple_Easy_Version.mid".to_owned());

    let mut key_presses: Vec<Message> = Vec::new();
    let mut data = [0; 256];
    data[0] = 9;
    data[1] = 128;
    data[2] = 54;
    data[3] = 64;
    
    key_presses.insert(0, Message::new(data, SystemTime::now() + Duration::new(5,0)));
    let mut key_pressed: Option<Message>;

    let threshold = Duration::from_millis(100);

    // Create a channel for communication between threads
    let (sender, receiver) = mpsc::channel();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("usage: read_device <base-10/0xbase-16> <base-10/0xbase-16>");
        return;
    }
    let vid = convert_argument(args[1].as_ref());
    let pid = convert_argument(args[2].as_ref());
    // Spawn a thread for reading data into the buffer
    thread::spawn(move || {
        match Context::new() {
            Ok(mut context) => match open_device(&mut context, vid, pid) {
                Some((mut device, device_desc, mut handle)) => {
                    read_device(&mut device, &device_desc, &mut handle, sender).unwrap()
                }
                None => println!("could not find device {:04x}:{:04x}", vid, pid),
            },
            Err(e) => panic!("could not initialize libusb: {}", e),
        }
    });

    println!("notes: {:?}", midi.messages.len());
    thread::sleep(Duration::from_millis(2000));
    let time_start = SystemTime::now();
    let mut current_message = 0;
    let mut note_hit = false;
    loop {
        let received_data = receiver.try_recv();
        if received_data.is_ok() {
            key_pressed = Some(received_data.unwrap());
        } else {
            key_pressed = None;
        }
        // ... sure
        // if key_pressed.is_some_and(|k| k.pressed_at > time_start + time_start.elapsed().unwrap()) {
        //     key_pressed = key_presses.pop();
        // }

        if time_start.elapsed().unwrap() >= midi.messages[current_message].play_at {
            println!("{}\t{}", current_message, midi.messages[current_message]);

            if !note_hit {
                println!("hit: Miss");
            }

            note_hit = false;
            current_message += 1;
        }

        if !note_hit && key_pressed.is_some() {
            let hit = key_pressed.unwrap().hit_accuracy(&midi.messages[current_message], threshold, 1, time_start);
            println!("hit: {}", hit as u8);
            note_hit = true;
        }

        if current_message >= midi.messages.len() {
            break;
        }
    }

    // MidiFile::new("Nintendo_Wii_Theme_for_Bb_Clarinet.mid".to_owned());
    // MidiFile::new("hail-mary/test.mid".to_owned());
    // TOPDO is length encoded faster. how would it work in python and stuff. test it now
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
        
    }

    println!("start");
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    None
}

fn read_device<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    handle: &mut DeviceHandle<T>,
    sender: mpsc::Sender<Message>,
) -> Result<()> {
    handle.reset()?;

    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);
    println!("Languages: {:?}", languages);

    if !languages.is_empty() {
        let language = languages[0];

        println!(
            "Manufacturer: {:?}",
            handle
                .read_manufacturer_string(language, device_desc, timeout)
                .ok()
        );
        println!(
            "Product: {:?}",
            handle
                .read_product_string(language, device_desc, timeout)
                .ok()
        );
        println!(
            "Serial Number: {:?}",
            handle
                .read_serial_number_string(language, device_desc, timeout)
                .ok()
        );
    }

    // TODO not sure what this does
    // match find_readable_endpoint(device, device_desc, TransferType::Interrupt) {
    //     Some(endpoint) => read_endpoint(handle, endpoint, TransferType::Interrupt, sender),
    //     None => println!("No readable interrupt endpoint"),
    // }

    match find_readable_endpoint(device, device_desc, TransferType::Bulk) {
        Some(endpoint) => read_endpoint(handle, endpoint, TransferType::Bulk, sender),
        None => println!("No readable bulk endpoint"),
    }

    Ok(())
}

fn find_readable_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<Endpoint> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == transfer_type
                    {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                        });
                    }
                }
            }
        }
    }

    None
}

fn read_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: Endpoint,
    transfer_type: TransferType,
    sender: mpsc::Sender<Message>,
) {
    println!("Reading from endpoint: {:?}", endpoint);

    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface).ok();
            true
        }
        _ => false,
    };

    println!(" - kernel driver? {}", has_kernel_driver);

    match configure_endpoint(handle, &endpoint) {
        Ok(_) => {
            let timeout = Duration::from_secs(1);

            // let (sender, receiver) = mpsc::channel();

            let mut buf = [0; 256];

            loop {
                match transfer_type {
                    TransferType::Interrupt => {
                        match handle.read_interrupt(endpoint.address, &mut buf, timeout) {
                            Ok(len) => {
                                println!(" - read: {:?}", &buf[..len]);
                                // let buffer = buf.clone(); // Clone or handle ownership appropriately
                                // sender.send(buffer.to_vec()).unwrap();
                            }
                            Err(err) => println!("could not read from endpoint: {}", err),
                        }
                    }
                    TransferType::Bulk => match handle.read_bulk(endpoint.address, &mut buf, timeout) {
                        Ok(len) => {
                            println!(" - read: {:?}", &buf[..len]);
                            let message = Message::new(buf, SystemTime::now());
                            println!("{}", message);
                            sender.send(message).unwrap();
                        }
                        Err(err) => println!("could not read from endpoint: {}", err),
                    },
                    _ => (),
                }

                // let buffer = buf.clone().to_vec(); 
                // sender.send(buffer).unwrap();
            }

        

            // loop {
            //     // Receive data from the buffer
            //     // let received_data = receiver.recv().unwrap();
        
            //     // Process the received data (replace with your processing logic)
            //     println!("Received: {:?}", received_data);
            // }
            
        }
        Err(err) => println!("could not configure endpoint: {}", err),
    }

    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface).ok();
    }
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
    Ok(())
}

struct State {
    duration: u64,
    ctx: Option<egui::Context>,
    stuff: Stuff,
}

struct Stuff {
    name: String,
    age: u32,
    time_start: SystemTime,
    midi: MidiFile,
    threshold: Duration,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    current_message: usize,
    note_hit: bool,
    notes_played: u128,
    total_notes: usize,
}

impl State {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let midi = MidiFile::new("Bad_Apple_Easy_Version.mid".to_owned());
        let total_notes = midi.messages.len();
        Self {
            duration: 0,
            ctx: None,
            stuff: Stuff {
                name: "Arthur".to_owned(),
                age: 42,
                time_start: SystemTime::now(),
                midi,
                threshold: Duration::from_millis(100),
                sender,
                receiver,
                current_message: 0,
                note_hit: false,
                notes_played: 0,
                total_notes,
            },
        }
    }
}

struct MyApp {
    state: Arc<Mutex<State>>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        state.lock().unwrap().ctx = Some(cc.egui_ctx.clone());
        let state_clone = state.clone();
        std::thread::spawn(move || {
            slow_process(state_clone);
        });
        
        Self {
            state,
        }
    }
}

fn slow_process(state_clone: Arc<Mutex<State>>) {
    // let received_data = self.receiver.try_recv();
    // let key_pressed = if received_data.is_ok() {
    //     Some(received_data.unwrap())
    // } else {
    //     None
    // };

    loop {
        let state = &mut state_clone.lock().unwrap();
        let stuff = &mut state.stuff;
        if stuff.time_start.elapsed().unwrap() >= stuff.midi.messages[stuff.current_message].play_at {
            println!("{} {} {:?} {:?}", stuff.current_message, stuff.midi.messages[stuff.current_message].note, stuff.midi.messages[stuff.current_message].play_at, stuff.time_start.elapsed().unwrap());

            if !stuff.note_hit {
                println!("hit: Miss");
            }
    
            // only push if on
            let status = stuff.midi.messages[stuff.current_message].status as u8;
            if status == 144 {
                // oh smart. this returns the type. so have to do this style
                stuff.notes_played |= 2_u128.pow((stuff.midi.messages[stuff.current_message].note-1).into())
            } else if status == 128 {
                // for i in 0..stuff.notes_played.len() {
                //     if stuff.midi.messages[stuff.current_message].note == stuff.notes_played[i] {
                //         stuff.notes_played&=stuff.midi.messages[stuff.current_message].note as u128;
                //     }
                // }
                stuff.notes_played &=! 2_u128.pow((stuff.midi.messages[stuff.current_message].note-1).into())
            }
    
            stuff.note_hit = false;
            stuff.current_message += 1;
        }
    
        let ctx =&state.ctx;
        match ctx {
            Some(x) => x.request_repaint(),
            None => panic!("error in Option<>"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let main_window = ctx.input(|i| i.viewport().outer_rect).unwrap();
        let width = main_window.width();
        let height = main_window.height();

        egui::CentralPanel::default().show(ctx, |ui| {
            let state = &mut self.state.lock().unwrap();
            let total_notes = state.stuff.total_notes;
            let health = 100;

            ui.heading(state.stuff.midi.name.clone());
            ui.hyperlink("https://github.com/JoeyShapiro/Oregano");
            ui.label(format!("Health: {health}%"));

            let progress = ( state.stuff.current_message as f32 / total_notes as f32 ) * width;
            let progress_bar = egui::Rect{ min: egui::pos2(0.0, 0.0), max: egui::pos2(progress, 10.0) };
            ui.painter()
                .rect_filled(progress_bar, 0.0, egui::Color32::GRAY);

            let play_height = height-75.0;
            for i in 0..2 {
                let cur_bar_pos = ((i as f32 / 2.0 * play_height) + (state.stuff.time_start.elapsed().unwrap().as_secs_f32() * 100.0)) % play_height;
                let bar_bar = egui::Rect{ min: egui::pos2(0.0, cur_bar_pos+0.0), max: egui::pos2(width, cur_bar_pos+2.0) };
                ui.painter()
                    .rect_filled(bar_bar, 0.0, egui::Color32::GRAY);
            }

            // Within each row rect, we paint the columns
            let cur_note = state.stuff.midi.messages[state.stuff.current_message].note as usize;
            let cur_notes = state.stuff.notes_played;
            for i in 0..=127 { // TODO this is wrong, missing the last note, but cant fit it
                let x = 0.0 + (i * 10) as f32;
                // TODO thats a lot of math
                let base_i = 2_u128.pow(i);
                let color = if cur_notes&base_i== base_i {
                    egui::Color32::RED
                } else if i % 2 == 0 {
                    egui::Color32::DARK_GRAY
                } else {
                    egui::Color32::GRAY 
                };
                // let color = if i == cur_note.note { egui::Color32::RED } else { color }; // TODO this line cuases crash
                // i think this is wrong, 50 should be bigger
                let rect = egui::Rect{ min: egui::pos2(x, height-75.0), max: egui::pos2(x + 10.0, height) };
                ui.painter()
                    .rect_filled(rect, 0.0, color);
            }
        });
    }
}
