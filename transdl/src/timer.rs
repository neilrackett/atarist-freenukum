use super::ll;
use std::os::raw::c_void;

unsafe extern "C" fn timer_elapsed<F>(
    interval: u32,
    param: *mut c_void,
) -> u32
where
    F: FnMut(),
{
    let param = &mut *(param as *mut F);
    param();
    interval
}

pub struct Timer {
    timer_id: ll::SDL_TimerID,
}

impl Timer {
    pub fn add<F: FnMut() + Sized>(interval: u32, mut f: F) -> Self {
        let cb_ptr = &mut f as *mut F as *mut c_void;

        let timer_id = unsafe {
            ll::SDL_AddTimer(interval, Some(timer_elapsed::<F>), cb_ptr)
        };

        Timer { timer_id }
    }

    pub fn remove(self) {
        unsafe { ll::SDL_RemoveTimer(self.timer_id) };
    }
}
