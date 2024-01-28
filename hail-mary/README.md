i dont need a single different
i can just see it change
i just need everyone change
right right right
i need to deal with delta time too, but this is good for now

```
...
40 00 5B 00 00 5D 00 00 FF 21 01 00 00 90 n5 v1
83 5F n5 00 01 n1 ?v 8? ?F n1 00 01 47 50 83 5F
47 00 01 n9 v5 83 5F n9 00 01 n6 v2 8? ?F n6 00
01 n2 ?v 83 5F n2 00 01 47 50 83 5F 47 00 01 na
v6 8? ?F na 00 01 n7 v3 83 5F n7 00 01 n3 ?v 83
5F n3 00 01 47 50 83 5F 47 00 01 nb v7 83 5F nb
00 01 n8 v4 8? ?F n8 00 01 n4 ?v 83 5F n4 00 01
47 50 83 5F 47 00 01 nc v8 83 5F nc 00 01 FF 2F
...
```
question marks are only changed. i should be able to see pattern now

this is missing the last 2, and first note. because those dont change
also, i think the messured values are only the second notes
actually im not sure, so i should change the first note, and see what changes
that should give me enough. i could guess the patter, but why bother
yeah i see im missing stuff. the `vv` is only for the second note
so i clipped it, and its only the velocity of the second. i htink i see it,
but this should help

yup i was right, this should be enought, but if not, change the last note

n1-4 = second
n5-8 = first
n9-c = forth

```
...
40 00 5B 00 00 5D 00 00 FF 21 01 
00 00 90 n1 vv
dd dd n1 00 01 n2 vv
dd dd n2 00 01 n3 vv
dd dd n3 00 01 n4 vv
dd dd n4 00 01 n5 vv

dd dd n5 00 01 n6 vv
dd dd n6 00 01 n7 vv
dd dd n7 00 01 n8 vv
dd dd n8 00 01 n9 vv

dd dd n9 00 01 na vv
dd dd na 00 01 nb vv
dd dd nb 00 01 nc vv
dd dd nc 00 01
FF 2F 00
...
```

```
...
00 90 n1 50 dd dd n1 00
dd dd n2 50 dd dd n2 00
dd dd n3 50 dd dd n3 00
dd dd n4 50 dd dd n4 00 dd 
...
```

if it doesnt have a status, then contineu
this works actually, because all status are above 0x80, and any that are, are undefiend

```
...
FF 21 01 00 87 40 90 44
50 81 1F 44 00 01 42 50 81 1F 42 00 01 44 50 81
1F 44 00 01 42 50 81 1F 42 00 01 3F 50 81 1F 3F
00 01 42 50 81 1F 42 00 01 3F 50 81 6F 3F 00 01
41 50 81 6F 41 00 01 42 50 81 6F 42 00 01 44 50
81 6F 44 00 01 46 50 83 5F 46 00 01 4B 50 81 6F
4B 00 01 49 50 81 6F 49 00 01
...
```

pages:
91: status

it grows, but they dont line up
but it did work in thinkg
nope, that changes but doesnt line up
that is it, they line up, but the octaves dont match
could find center point, but whatever


y=\operatorname{floor}\left(\frac{x}{12}\right)-\left(k-3\right)

could try to stream it. get each start, then go through it, but still need when to play. cant multie thread
but how will i get delta time of different channels
its gonan be a game, so i will have to loop time anyway

this is good enough, i can figure out the rest on rust
