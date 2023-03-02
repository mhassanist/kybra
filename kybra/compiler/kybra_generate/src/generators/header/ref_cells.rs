use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        thread_local! {
            static RNG_REF_CELL: std::cell::RefCell<StdRng> = std::cell::RefCell::new(SeedableRng::from_seed([0u8; 32]));
        }
    }
}
