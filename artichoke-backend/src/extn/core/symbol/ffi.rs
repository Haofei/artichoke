use std::ffi::{CStr, c_char};
use std::ptr;
use std::slice;

use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

// ```c
// MRB_API mrb_sym mrb_intern(mrb_state*,const char*,size_t);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern(mrb: *mut sys::mrb_state, name: *const c_char, len: usize) -> sys::mrb_sym {
    let bytes = slice::from_raw_parts(name.cast::<u8>(), len);
    let bytes = bytes.to_vec();
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes(bytes);
    sym.unwrap_or_default()
}

// ```c
// MRB_API mrb_sym mrb_intern_static(mrb_state*,const char*,size_t);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_static(
    mrb: *mut sys::mrb_state,
    name: *const c_char,
    len: usize,
) -> sys::mrb_sym {
    let bytes = slice::from_raw_parts::<'static, _>(name.cast::<u8>(), len);
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes(bytes);
    sym.unwrap_or_default()
}

// ```c
// MRB_API mrb_sym mrb_intern_cstr(mrb_state *mrb, const char* str);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_cstr(mrb: *mut sys::mrb_state, name: *const c_char) -> sys::mrb_sym {
    let string = CStr::from_ptr(name);
    let bytes = string.to_bytes_with_nul().to_vec();
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes_with_trailing_nul(bytes);
    sym.unwrap_or_default()
}

// ```c
// MRB_API mrb_sym mrb_intern_str(mrb_state*,mrb_value);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_str(mrb: *mut sys::mrb_state, name: sys::mrb_value) -> sys::mrb_sym {
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let name = Value::from(name);
    let Ok(bytes) = name.try_convert_into_mut::<Vec<u8>>(&mut guard) else {
        return 0;
    };
    let sym = guard.intern_bytes(bytes);
    sym.unwrap_or_default()
}

/* `mrb_intern_check` series functions returns 0 if the symbol is not defined */

// ```c
// MRB_API mrb_sym mrb_intern_check(mrb_state*,const char*,size_t);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_check(
    mrb: *mut sys::mrb_state,
    name: *const c_char,
    len: usize,
) -> sys::mrb_sym {
    let bytes = slice::from_raw_parts(name.cast::<u8>(), len);
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let Ok(Some(sym)) = guard.check_interned_bytes(bytes) else {
        return 0;
    };
    sym
}

// ```c
// MRB_API mrb_sym mrb_intern_check_cstr(mrb_state*,const char*);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_check_cstr(mrb: *mut sys::mrb_state, name: *const c_char) -> sys::mrb_sym {
    let string = CStr::from_ptr(name);
    let bytes = string.to_bytes_with_nul();
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let Ok(Some(sym)) = guard.check_interned_bytes_with_trailing_nul(bytes) else {
        return 0;
    };
    sym
}

// ```c
// MRB_API mrb_sym mrb_intern_check_str(mrb_state*,mrb_value);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_intern_check_str(mrb: *mut sys::mrb_state, name: sys::mrb_value) -> sys::mrb_sym {
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let name = Value::from(name);
    let Ok(bytes) = name.try_convert_into_mut::<&[u8]>(&mut guard) else {
        return 0;
    };
    let Ok(Some(sym)) = guard.check_interned_bytes(bytes) else {
        return 0;
    };
    sym
}

// `mrb_check_intern` series functions returns `nil` if the symbol is not
// defined; otherwise returns `mrb_value`.

// ```c
// MRB_API mrb_value mrb_check_intern(mrb_state*,const char*,size_t);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_check_intern(
    mrb: *mut sys::mrb_state,
    name: *const c_char,
    len: usize,
) -> sys::mrb_value {
    let bytes = slice::from_raw_parts(name.cast::<u8>(), len);
    unwrap_interpreter!(mrb, to => guard);
    let Ok(Some(sym)) = guard.check_interned_bytes(bytes) else {
        return Value::nil().inner();
    };
    Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default().inner()
}

// ```c
// MRB_API mrb_value mrb_check_intern_cstr(mrb_state*,const char*);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_check_intern_cstr(mrb: *mut sys::mrb_state, name: *const c_char) -> sys::mrb_value {
    let string = CStr::from_ptr(name);
    let bytes = string.to_bytes_with_nul();
    unwrap_interpreter!(mrb, to => guard);
    let Ok(Some(sym)) = guard.check_interned_bytes_with_trailing_nul(bytes) else {
        return Value::nil().inner();
    };
    Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default().inner()
}

// ```c
// MRB_API mrb_value mrb_check_intern_str(mrb_state*,mrb_value);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_check_intern_str(mrb: *mut sys::mrb_state, name: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let name = Value::from(name);
    let Ok(bytes) = name.try_convert_into_mut::<&[u8]>(&mut guard) else {
        return Value::nil().inner();
    };
    let Ok(Some(sym)) = guard.check_interned_bytes(bytes) else {
        return Value::nil().inner();
    };
    Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default().inner()
}

// ```c
// MRB_API const char *mrb_sym_name(mrb_state*,mrb_sym);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_sym_name(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> *const c_char {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    let Ok(Some(bytes)) = guard.lookup_symbol_with_trailing_nul(sym) else {
        return ptr::null();
    };
    bytes.as_ptr().cast::<c_char>()
}

// ```c
// MRB_API const char *mrb_sym_name_len(mrb_state*,mrb_sym,mrb_int*);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_sym_name_len(
    mrb: *mut sys::mrb_state,
    sym: sys::mrb_sym,
    lenp: *mut sys::mrb_int,
) -> *const c_char {
    if !lenp.is_null() {
        ptr::write(lenp, 0);
    }
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    let Ok(Some(bytes)) = guard.lookup_symbol(sym) else {
        return ptr::null();
    };
    if !lenp.is_null() {
        let Ok(len) = sys::mrb_int::try_from(bytes.len()) else {
            return ptr::null();
        };
        ptr::write(lenp, len);
    }
    bytes.as_ptr().cast()
}

// ```c
// MRB_API const char *mrb_sym_dump(mrb_state*,mrb_sym);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_sym_dump(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> *const c_char {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    let Ok(Some(bytes)) = guard.lookup_symbol(sym) else {
        return ptr::null();
    };
    let bytes = bytes.to_vec();
    // Allocate a buffer with the lifetime of the interpreter and return
    // a pointer to it.
    let Ok(string) = guard.try_convert_mut(bytes) else {
        return ptr::null();
    };
    let Ok(bytes) = string.try_convert_into_mut::<&[u8]>(&mut guard) else {
        return ptr::null();
    };
    bytes.as_ptr().cast()
}

// ```c
// MRB_API mrb_value mrb_sym_str(mrb_state*,mrb_sym);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_sym_str(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);

    let value = if let Ok(Some(bytes)) = guard.lookup_symbol(sym) {
        let bytes = bytes.to_vec();
        guard.try_convert_mut(bytes)
    } else {
        guard.try_convert_mut("")
    };
    value.unwrap_or_default().inner()
}

// ```c
// void mrb_init_symtbl(mrb_state*);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_init_symtbl(mrb: *mut sys::mrb_state) {
    // The symbol table is initialized before the call to `mrb_open_allocf` in
    // `crate::interpreter::interpreter`. This function is intended to be called
    // during the initialization of the `mrb_state`.
    let _ = mrb;
}

// ```c
// void mrb_free_symtbl(mrb_state *mrb);
// ```
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn mrb_free_symtbl(mrb: *mut sys::mrb_state) {
    // The symbol table is freed when the Rust `State` is freed.
    let _ = mrb;
}
