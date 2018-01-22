#![crate_type = "lib"]
extern crate libc;
pub mod test_struct;


fn fold(x:i32,ys:&[i32])->i32{
    ys.iter().map(|&e| e * x).sum()
}

#[no_mangle]
pub extern "C" fn fold_c(x:libc::c_int,ys:*const libc::c_int,ys_len:libc::size_t)->libc::c_int{
    let ys = if ys.is_null(){
        eprintln!("error");
        return 0;
    }else{
        unsafe{std::slice::from_raw_parts(ys,ys_len)}
    };
    fold(x,ys)
}

pub struct Folder{
    seed:Vec<i32>,
}
impl Folder{
    fn call(&self,x:i32)->i32{
        self.seed.iter().map(|&e| e * x ).sum()
    }
}

#[no_mangle]
pub extern "C" fn new_folder(ys:*const libc::c_int,length:libc::size_t)->*mut Folder{
    let ys = if ys.is_null(){
        eprintln!("error");
        return std::ptr::null_mut();
    }else{
        unsafe{std::slice::from_raw_parts(ys,length)}
    };
    let res = Folder{seed:ys.to_vec()};
    Box::into_raw(Box::new(res))
}
#[no_mangle]
pub extern "C" fn fold_struct(x:libc::c_int,folder:*mut Folder)->libc::c_int{
    let folder = if folder.is_null(){
        eprintln!("error");
        return 0;
    }else{
        unsafe{&mut *folder}
    };
    folder.call(x)
}

#[no_mangle]
pub extern "C" fn free_struct(folder:*mut Folder){
    let _folder = if folder.is_null(){
        eprintln!("error");
        return ;
    }else{
        unsafe{Box::from_raw(folder)}
    };
}

