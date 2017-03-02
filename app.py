#!/usr/bin/env python3

from ctypes import byref, cdll, c_size_t, POINTER, Structure

lib = cdll.LoadLibrary("target/debug/libpython_example.dylib")

class LevDistCache(Structure):
    pass

ldc_new = lib.ldc_new
ldc_new.restype = POINTER(LevDistCache)

ldc_delete = lib.ldc_delete
ldc_delete.restype = None

cache = ldc_new()
print("Allocated the cache")

for _ in range(10):
    left = b"wutang"
    right = b"mutate"

    distance = c_size_t(0)
    ok = lib.ldc_distance(cache,
                          left,
                          len(left),
                          right,
                          len(right),
                          byref(distance))

    if ok != 0:
        print("Uh oh, something went wrong")
        break
    else:
        print("The Levenshtein distance between {} and {} is {}".format(
            left, right, distance))

ldc_delete(cache)
cache = None

print("Deallocated the cache")
