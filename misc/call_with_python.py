

def using_ctypes():
    print("[py] Running with ctypes")

    from ctypes import cdll

    lib = cdll.LoadLibrary("../target/release/libffitest.so")

    print("[py] Printing...")
    lib.print_something()

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

        typedef struct {
            float x;
            float y;
        } Point2;

        typedef struct {
            float x;
            float y;
        } Vector2;

        typedef struct {
            Point2 position;
            Vector2 velocity;
            float mass;
            float radius;
        } Body;

    """)
    #Something = ffi.new("struct Something *")
    #Something.i = ffi.new("int")
    #Something.j = ffi.new("int")

    ffi.cdef("""
        void print_something();

        void process();

        int return_int_plus_one(int i);

        Something return_something(int i, int j);

        Body create_body();
    """)
    lib = ffi.dlopen("../target/release/libffitest.so")

    print("[py] Print from Rust")
    lib.print_something()

    #lib.process()

    print("[py] Simple checks")
    assert lib.return_int_plus_one(1) == 2

    print("[py] Getting something...")
    something = lib.return_something(1, 2)
    print(something.i)
    print(something.j)

    body = lib.create_body()
    print(body.position.x)
    print(body.position.y)


if __name__ == "__main__":
    using_ctypes()
    using_cffi()
    print("[py] done!")
