0FFFFFFF	FF FF FF 7F
isee isee isee
left is delta, and right is in file rep
the last bit is 0, so this is max value
since the final bit is always set, you ignore it, changing the whole number
this is actually easy

now this is how i do it. reading the file. looking for patterns

i can look for pattern. the notes dropping.
i could keep chaning one thing, and see what happens. hail mary of hail marys
but i was trying to see how many times a note appeared
or search for a byte to drop and see it. or go backwards
this is the delta time

look at 0x47. it is 7B apart. for each one. hopefully its the delta

this works i guess
I FOUND IT

```
00 00 FF 21 01 00 00 90 54

50 83 47 54 00 19 | 48
50 83 47 48 00 19 3C
50 83 47 3C 00 19 54

50 83 47 54 00 19 48
50 83 47 48 00 19 3C
50 83 47 3C 00 19 54

50 83 47 54 00 19 48
50 83 47 48 00 19 3C
50 83 47 3C 00 83 79

3C 50 00 48 | 50 00 54
50 83 47 3C 00 00 48
00 00 54 00 01
FF 2F 00
```

of course, i need to get different values. i will be off
ah, it was goind where it shouldnt, leading to issue
this is fine, they have same code anyway

it was 1 off, but it was ok alone, so it didnt matter
couold do the symbol, but its fine. better this way
oh cuase those are even smaller, but this is milli, i see now

i shouold make sure they dont just hold the key. but i can do that in later testing.
also, they will have to let go of keys and press different keys. it shouldnt be a problem
at least not now
the hit will sum it up. but i can meassure a few things
- Note
- timing
- velocity
- octave
- ... other stuff i dont know ...

might have to make a queue for notes being pressed

looks pretty clean

want queue for control
i wonder how to deal with handing messages, but whatevs, some other time

but how can i deal with stale inputs

actually, i could just add them in the proper loop

looks like the recv waits. like classic sockets
try might be like pop. its close enough
but how could i put it into an array and try to pop the values. i think this just does that dumb stuff

hav to clean up but maybe wait, want to see gui

24 1 240
25 1 250
thread 'main' panicked at src/main.rs:426:69:
attempt to multiply with overflow
```rust
let cur_note = self.midi.messages[self.current_message].note;

            for i in 0..=128 {
                println!("{} {} {}", i, self.current_message, 0.0 + (i * 10) as f32);
                let x = 0.0 + (i * 10) as f32;
                let color = if i == cur_note {
                    egui::Color32::RED
                } else if i % 2 == 0 {
                    egui::Color32::DARK_GRAY
                } else {
                    egui::Color32::GRAY 
                };
                // let color = if i == cur_note.note { egui::Color32::RED } else { color }; // TODO this line cuases crash
                let rect = egui::Rect{ min: egui::pos2(x, 200.0), max: egui::pos2(x + 10.0,50.0 + 200.0) };
                ui.painter()
                    .rect_filled(rect, 0.0, color);
            }
```
the type of i in now inferred as u8, so not i itself, but trying to get 260 will cause overflow

i htink now its getting hung on something
it only updates when i move mouse -_-

oh, i dont think the ui can handle multiple notes being hit at the same time. only 1 will end up lighting up.
so if they are close it wont work right

maybe use a list, but doesnt super matter. need a tile chart
could just use time to say how big they are

ahhh, i need 129 bits. stupid midi
still have an overflow

could make a byte or 2 for each octave. and do a list, i mean array

can it draw ui as read while time writes next element
no, it has a lock. it has to stop and wait
not sure if write is correct, but good enough
the note is unknown?

why is there a minor miss align. when does it happen
i think i know why, the slow change
i found the notes before. what notes is counted as unknown

oh i kinda need the delta time
it seems like a little extra is in there. i wonder how that works
i htink both do it, but might be just that one
does it a few times, but some seem ok. just the big block

they all got realigned
i think i fixed it
need to get palyed notes, maybe receiver
and need a chart. hmmm

could compare the whole thing. but wouldtn give fine details
could store things, but lets do some graphics programming

yeah i need to go though the actual events. but i can sort them and stuff

i need some way to tell length of note
the note on and off stuff
i could convert all messages to notes, and keep record, but hmmm
i might be able to be clever; knowing that a note cannot be turned ON twice in a row

in 1d i toggle on and off. but now i have to show
not sure if its reversed or not, the x and y
it looks odd at the beginning, but then makes sense
maybe the notes are too close or something
