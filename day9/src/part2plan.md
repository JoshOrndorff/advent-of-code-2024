
000......11......2.3333.444.55555

Track the file locations in vec and the free spaces in another

Files:

Starts | Length | File Id - also index in Vector
-------|--------|----
0      |      3 | 0
9      |      2 | 1
17     |      1 | 2
19     |      4 | 3
24     |      3 | 4
28     |      5 | 5


Free Spaces: Stored in an ordered list, but indices are not important - just the ordering

Starts | Length
-------|--------
3      | 6
11     | 6
18     | 1
23     | 1
27     | 1

Start iterating the indices of the files from back to front.
First up is file id 5 which has a length of five.
To find its new home we start iterating the free spaces from the front.
The very first one available is length 6, and it will work as a new home.
So we update both tables.

The file now starts where the free space used to.
    files[id].start = free_space.start

The free space is smaller and starts farther to the right.
    free_space.start += file.length
    free_space.length -= file.length

We are now done with the iteration over free space. Exit early and start again from the beginning next time.

---

Move on to the next file which is id 4 and has a length of 3.
The first available free space starts at index five and has a length of 1. This is not long enough.
The next free space starts at 11 and has a length of six. This is enough.

The file moves to start at position 11.
The freespace starts at 14 and its length is reduced to 3.

Wubbalubbadubdub