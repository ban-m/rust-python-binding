import concurrent.futures
from concurrent.futures import ProcessPoolExecutor
from cffi import FFI

ffi = FFI()
ffi.cdef("""
int fold_c(int x,const int* ys,size_t ys_len);
typedef void* Folder;
Folder new_folder(const int* ys,size_t length);
int fold_struct(int x,Folder ys);
void free_struct(Folder ys);
""")
libtest = ffi.dlopen("./target/release/libtest.dylib")

def fold_rust(x,ys):
    length = len(ys)
    ys = ffi.new("int[]",ys)
    return (x,libtest.fold_c(x,ys,length))

testseed = ffi.new("int[]",[x for x in range(8)])
testFold = libtest.new_folder(testseed,len(testseed))
def fold_struct(x):
    return (x,libtest.fold_struct(x,testFold))

def fold(x,ys):
    return (x,sum([x * y for y in ys]))

class Caller(object):
    def __init__(self,seed):
        self.seed = seed
        self.rust_struct = libtest.new_folder(self.seed,len(self.seed))
        global fold_by_struct
        def fold_by_struct(x):
            return (x,libtest.fold_struct(x,self.rust_struct))

    def call(self,xs):
        with ProcessPoolExecutor() as executor:
            futures = [executor.submit(fold_by_struct,x) for x in xs]
            for future in concurrent.futures.as_completed(futures):
                (x,result) = future.result()
                print("{} -> {}".format(x,result))

    def __del__(self):
        libtest.free_struct(self.rust_struct)

if __name__ == '__main__':
    c = Caller([x for x in range(8)])
    c.call([1,2,3])
