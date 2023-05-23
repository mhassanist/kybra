use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn trap(
            &self,
            message_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine,
        ) -> rustpython_vm::PyResult {
            let message: String = message_py_object_ref
                .try_from_vm_value(vm)
                .map_err(|try_from_err| vm.new_type_error(try_from_err.0))?;

            ic_cdk::api::trap(&message)
        }
    }
}
