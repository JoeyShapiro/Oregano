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
