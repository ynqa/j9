use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use j9_sys::{
    jq_compile, jq_init, jq_next, jq_start, jq_teardown, jv, jv_copy, jv_dump_string, jv_free,
    jv_get_kind, jv_kind_JV_KIND_INVALID, jv_parse, jv_string_value,
};

mod error;

pub use error::{Error, Result};

fn jv_is_valid(x: jv) -> bool {
    unsafe { jv_get_kind(x) != jv_kind_JV_KIND_INVALID }
}

unsafe fn get_string_from_c_char(x: *const c_char) -> Result<String> {
    Ok(CStr::from_ptr(x).to_str()?.to_string())
}

pub fn run(program: &str, input: &str) -> Result<Vec<String>> {
    let mut state = unsafe { jq_init() };
    if state.is_null() {
        return Err(Error::JqInitError);
    }

    let input_cstring = CString::new(input)?;
    let input_jv = unsafe { jv_parse(input_cstring.as_ptr()) };
    if !jv_is_valid(input_jv) {
        return Err(Error::JvParseError(input.to_string()));
    }

    let program_cstring = CString::new(program)?;
    if unsafe { jq_compile(state, program_cstring.as_ptr()) } == 0 {
        return Err(Error::JqCompileError(program.to_string()));
    };

    unsafe {
        jq_start(state, jv_copy(input_jv), 0);
    };

    let mut ret = vec![];

    unsafe {
        let mut cur = jq_next(state);

        while jv_is_valid(cur) {
            let dumped = jv_dump_string(jv_copy(cur), 0);
            let str = get_string_from_c_char(jv_string_value(dumped))?;

            ret.push(str);
            jv_free(cur);

            cur = jq_next(state);
        }
    }

    unsafe {
        jv_free(input_jv);
        jq_teardown(&mut state);
    }

    Ok(ret)
}
