# could read xxd -p file.mid > file.hex; # but that is too much work
import glob

num = [0x83, 0x57]
num = [ 0x87, 0x3f ]
# num = [0xff, 0x7f]
i = 0
delta = num[i]&0b0111_1111
while num[i]&0b1000_0000 == 0x80:
    print('hello')
    i+=1
    delta = delta<<7|(num[i]&0b0111_1111)

print(delta, f"0x{delta:04X}")
# exit(0)

files = glob.glob('*.mid')
for file in files:
    with open(file, 'rb') as f:
        data = f.read()

    formatted = []
    for i, b in enumerate(data):
        formatted.append(f"{b:02X}")

    with open(f"{file[:-4]}.hex", 'w') as f:
        f.write(f"length: {len(data)}\n")
        for line in [ formatted[i:i+16] for i in range(0, len(formatted), 16) ]:
            f.write(' '.join(line)+'\n')
