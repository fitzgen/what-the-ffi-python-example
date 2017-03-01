#!/usr/bin/env python3

from ctypes import byref, cdll, c_size_t

lib = cdll.LoadLibrary("target/debug/libpython_example.dylib")

left = b"wutang"
right = b"mutate"

distance = c_size_t(0)
ok = lib.levenshtein_distance(left,
                              len(left),
                              right,
                              len(right),
                              byref(distance))

if ok != 0:
    print("Uh oh, something went wrong")
else:
    print("The levenshtein distance between {} and {} is {}".format(
        left, right, distance))
