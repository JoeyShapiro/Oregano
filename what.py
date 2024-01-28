import json
notes = 10+4+2+14+4+20+12+6+26+2+20+6+20+18+8+20+10+4
types = {}
eiths = 0
forts = 0

with open('Simple_MIDI.mid', 'rb') as f:
    lines = f.readlines()
    data = lines[0] + lines[1]
    print(len(data))
    for i, b in enumerate(data):
        if b not in types:
            types[b] = 1
        else:
            types[b]+=1
        if b == 0x50:
            eiths +=1
            print(f"status: 0x{data[i-2]&0b1111_0000:02X}\tchannel: {data[i-2]&0b0000_1111}\tnote: {data[i-1]}\tvelocity: {data[i]}")
        if b == 0x40:
            forts +=1
            print(f"status: 0x{data[i-2]&0b1111_0000:02X}\tchannel: {data[i-2]&0b0000_1111}\tnote: {data[i-1]}\tvelocity: {data[i]}")


print(json.dumps(types, indent='\t'))

for t in types:
    # if types[t] > notes:
    print(f"0x{t:02X}\t{types[t]}")

print('\nsorted')
for t in sorted(types.keys()):
    print(f"0x{t:02X}\t{types[t]}")

print('notes', notes)
print('eiths', eiths)
print('forts', forts)