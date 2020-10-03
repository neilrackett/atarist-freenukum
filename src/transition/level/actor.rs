use super::super::geometry::Geometry;

#[repr(C)]
#[derive(Default)]
pub struct ActorData {
    pub position: Geometry,
    pub is_in_foreground: bool,
}

pub mod ffi {
    type FnLevelActorData = super::ActorData;

    #[no_mangle]
    pub extern "C" fn fn_level_actor_data_create() -> *mut FnLevelActorData
    {
        Box::into_raw(Box::new(FnLevelActorData::default()))
    }

    #[no_mangle]
    pub extern "C" fn fn_level_actor_data_free(
        ptr: *mut FnLevelActorData,
    ) {
        if !ptr.is_null() {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
