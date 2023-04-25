use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[pymethod]
        fn _kybra_msg_cycles_accept(
            &self,
            max_amount_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyObjectRef {
            let max_amount: u64 = max_amount_py_object_ref.try_from_vm_value(vm).unwrap();
            ic_cdk::api::call::msg_cycles_accept(max_amount).try_into_vm_value(vm).unwrap()
        }
    }
}
