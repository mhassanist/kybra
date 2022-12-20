use crate::py_ast::kybra_types::KybraStmt;
use cdk_framework::ActDataType;

use rustpython_parser::ast::Mod;

use super::KybraProgram;

// TODO all variables should be called stable_b_tree_map_nodes
#[derive(Clone)]
pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: ActDataType,
    pub value_type: ActDataType,
    pub max_key_size: u32,
    pub max_value_size: u32,
}

impl KybraProgram<'_> {
    pub fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    }
                    .is_stable_storage()
                })
                .map(|stmt_kind| {
                    KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    }
                    .as_stable_storage()
                })
                .collect(),
            _ => vec![],
        }
    }
}
