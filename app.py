#!/usr/bin/env python3

from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libpython_example.dylib")

answer = lib.life_universe_everything()

print("The answer to life, the universe, and everything is {}".format(answer))
