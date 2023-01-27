use cdk_framework::nodes::{ActCanisterMethod, ActExternalCanister};

use crate::{
    generators::{
        async_result_handler, ic_object, rng_seed, stable_b_tree_map, unwrap_rust_python_result,
    },
    py_ast::kybra_types::StableBTreeMapNode,
};

pub fn generate(
    canister_methods: &Vec<ActCanisterMethod>,
    external_canisters: &Vec<ActExternalCanister>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> proc_macro2::TokenStream {
    let ic_object = ic_object::generate(
        canister_methods,
        external_canisters,
        stable_b_tree_map_nodes,
    );
    let unwrap_rust_python_result = unwrap_rust_python_result::generate();
    let async_result_handler = async_result_handler::generate(&external_canisters);
    let stable_b_tree_map = stable_b_tree_map::generate(stable_b_tree_map_nodes);
    let rng_seed = rng_seed::generate();

    quote::quote! {
        #ic_object
        #unwrap_rust_python_result
        #async_result_handler
        #stable_b_tree_map
        #rng_seed
    }
}
