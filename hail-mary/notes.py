import time
import math

with open('hail-mary/test.mid', 'rb') as f:
    data = f.read()

start = False
notes = []
indexes = [i for i, n in enumerate(data) if n == 0x90]
idx = 0

convert = {
    0: 'C',
    1: 'C#',
    2: 'D',
    3: 'D#',
    4: 'E',
    5: 'F',
    6: 'F#',
    7: 'G',
    8: 'G#',
    9: 'A',
    10: 'A#',
    11: 'B'
}

middle_octave = 4

# good enough, just testing
i = indexes[idx]-1
idx+=1
total = 0
while i < len(data):
    delta = data[i]&0b0111_1111
    while data[i]&0b1000_0000 == 0x80:
        i+=1
        delta = delta<<7|(data[i]&0b0111_1111)
    i+=1
    print(i, hex(data[i]))
    
    if data[i] == 0xff and data[i+1] == 0x2f and data[i+2] == 0x00:
        start = False
        print('end of track')
        total += delta * (500_000/480) / 1000
        notes.append({'end': True, 'detla': 0, 'time': total/1000, 'channel': idx-1})
        try:
            i = indexes[idx]
            idx+=1
        except:
            break

    # this here
    if data[i] == 0x90:
        total = 0
        start = True
        print('start')
        i+=1

    if start:
        # equation attempts:
        """
        round(data[i]/middle_c)*middle_octave
        # seems fine this way
        floor(x/12)-(k-3)
        """
        note_symbol = f"{convert[data[i]%12]}{math.floor(data[i]/12)-(middle_octave-3)}"
        ticks = delta * (500_000/480) / 1000
        total += ticks
        notes.append({
            'channel': idx-1,
            'delta': delta,
            'time': total/1000,
            'note': f"{note_symbol} ({data[i]})",
            'velocity': data[i+1],
            'on': True if data[i+1] != 0 else False
        })
        i+=1
    i+=1

# for i, note in enumerate(notes):
#     if 'end' in note:
#         print(f"{i+1}\t#################### end #################")
#     else:
#         print(f"{i+1}\t{round(ticks/1000, 3)}s\t\033[{'32' if note['on'] == True else '31'}m{note}\033[0m")

notes = sorted(notes, key=lambda x: x['time'])

time_start = time.time()
current_time = 0
cur_note = 0
while True:
    current_time = time.time()-time_start
    if current_time >= notes[cur_note]['time']:
        if 'end' in notes[cur_note]:
            print(f"{cur_note+1}\t#################### end channel {notes[cur_note]['channel']} #################")
        else:
            print(f"{cur_note+1}\t{round(ticks/1000, 3)}s\t\033[{'32' if notes[cur_note]['on'] == True else '31'}m{notes[cur_note]}\033[0m")
        cur_note+=1
    
    if current_time >= total/1000:
        break
