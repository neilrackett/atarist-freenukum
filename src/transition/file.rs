pub struct File(std::fs::File);

impl File {
    pub fn open(name: &str) -> std::io::Result<Self> {
        Ok(File(std::fs::File::open(name)?))
    }
}

mod ffi {
    type FnFile = super::File;

    #[no_mangle]
    pub unsafe extern "C" fn fn_open_file() -> *mut FnFile {
        match FnFile::open("uiae") {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }
}
