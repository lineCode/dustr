use crate::helpers::*;
use ::syn::*;

/// The std lib's `String` type behavior.
///
/// We currently use a `CString` from to ingest all `String` values. This might be unsafe if the string
/// was instantiated by the caller without using rust's instanciation mechanism. In that case, it
/// would be safe to use a `CStr`. See https://doc.rust-lang.org/std/ffi/struct.CString.html
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "String")
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type) -> String { todo!() }
    fn native(&self, _sty: &Type) -> String { todo!() }
    fn annotation(&self, _sty: &Type) -> Option<String> { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }

    fn imports(&self, _sty: &Type, _pkg: &str) -> Vec<String> { todo!() }
}
