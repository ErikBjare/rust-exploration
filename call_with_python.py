from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libffitest.so")

print("[py] Printing...")
lib.do_something()

print("[py] Counting...")
lib.process()

print("[py] done!")
