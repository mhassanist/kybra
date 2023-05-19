use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn accept_message(
            &self,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            ic_cdk::api::call::accept_message()
                .try_into_vm_value(vm)
                .map_err(|try_into_err| vm.new_type_error(try_into_err.0))
        }
    }
}
