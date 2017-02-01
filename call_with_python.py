

def using_ctypes():
    print("[py] Running with ctypes")

    from ctypes import cdll

    lib = cdll.LoadLibrary("target/release/libffitest.so")

    print("[py] Printing...")
    lib.do_something()

    print("[py] Counting...")
    lib.process()


def using_cffi():
    print("[py] Running with CFFI")

    import cffi

    from cffi import FFI
    ffi = FFI()
    #ffi.cdef("""
    #   int main_like(int argv, char *argv[]);
    #""")
    ffi.cdef("""
       typedef struct {
            int i;
            int j;
        } Something;
    """)
    #Something = ffi.new("struct Something *")
    #Something.i = ffi.new("int")
    #Something.j = ffi.new("int")

    ffi.cdef("""
       void do_something();

       int return_int(int i);

       Something return_something(int i, int j);
    """)
    lib = ffi.dlopen("target/release/libffitest.so")

    lib.do_something()

    print("[py] Getting something...")
    something = lib.return_something(1, 2)
    print(something.i)
    print(something.j)


if __name__ == "__main__":
    using_ctypes()
    using_cffi()
    print("[py] done!")
