pub struct File(pub std::fs::File);

impl File {
    pub fn open(name: &str) -> std::io::Result<Self> {
        println!("Opening file {:?}", name);
        Ok(File(std::fs::File::open(name)?))
    }

    pub fn as_ref_mut(&mut self) -> &mut std::fs::File {
        &mut self.0
    }
}

pub mod ffi {
    use libc::{c_char, c_void, size_t};
    use std::ffi::CStr;

    pub type FnFile = super::File;

    #[no_mangle]
    pub extern "C" fn fn_file_open(path: *const c_char) -> *mut FnFile {
        assert!(!path.is_null());
        let filename = {
            match unsafe { CStr::from_ptr(path) }.to_str() {
                Ok(filename) => filename,
                Err(e) => {
                    eprintln!("Couldn't read file name: {:?}.", e);
                    return std::ptr::null_mut();
                }
            }
        };

        match FnFile::open(filename) {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn fn_file_read(
        ptr: *mut FnFile,
        buffer: *mut c_void,
        length: size_t,
    ) {
        assert!(!ptr.is_null());

        let file = unsafe { &mut (*ptr) };

        assert!(!buffer.is_null());
        let mut buffer: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(
                buffer as *mut u8,
                length as usize,
            )
        };

        use std::io::Read;
        file.as_ref_mut().read_exact(&mut buffer).unwrap();
    }

    #[no_mangle]
    pub extern "C" fn fn_file_free(ptr: *mut FnFile) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
