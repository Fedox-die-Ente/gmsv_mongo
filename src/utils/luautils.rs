use rglua::prelude::{lua_newuserdata, lua_touserdata, LuaState};

pub fn write_userdata<T>(l: LuaState, data: T) {
    let data_ptr = lua_newuserdata(l, std::mem::size_of::<T>()) as *mut T;
    unsafe {
        std::ptr::write(data_ptr, data);
    }
}

pub fn read_userdata<T: Clone>(l: LuaState) -> Result<T, String> {
    let data_ptr = lua_touserdata(l, 1) as *mut T;
    if data_ptr.is_null() {
        Err("Invalid userdata.".to_string())
    } else {
        Ok(unsafe { (*data_ptr).clone() })
    }
}