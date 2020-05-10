use ::syn::*;

/// Any unknown type's behavior, assumed to implement an ffi shim themselves.
///
/// If a type is unknown (meaning it did not match any pre-defined `Behavior`,) we consider it by
/// default to be a `Foreign` type: a type defined by the user that itself implements the ffi shim.
pub struct BehaviorForeign;

impl super::Behavior for BehaviorForeign {
    fn is(&self, _: &Type) -> bool {
        true
    }

    fn ffi(&self, _sty: &Type) -> String { todo!() }
    fn native(&self, _sty: &Type) -> String { todo!() }
    fn annotation(&self, _sty: &Type) -> Option<String> { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }

    fn imports(&self, _sty: &Type, _pkg: &str) -> Vec<String> { todo!() }
}
