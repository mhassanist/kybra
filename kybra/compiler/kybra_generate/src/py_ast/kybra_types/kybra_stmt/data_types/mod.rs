mod records;
mod tuples;

use rustpython_parser::ast::StmtKind;

use crate::cdk_act::ActDataType;

use super::KybraStmt;

impl KybraStmt<'_> {
    pub fn build_act_data_type(&self) -> ActDataType {
        match &self.stmt_kind.node {
            StmtKind::ClassDef { .. } => self.as_record(),
            StmtKind::FunctionDef { .. } => todo!(),
            StmtKind::AsyncFunctionDef { .. } => todo!(),
            StmtKind::Return { .. } => todo!(),
            StmtKind::Delete { .. } => todo!(),
            StmtKind::Assign { .. } => self.as_tuple(),
            StmtKind::AugAssign { .. } => todo!(),
            StmtKind::AnnAssign { .. } => todo!(),
            StmtKind::For { .. } => todo!(),
            StmtKind::AsyncFor { .. } => todo!(),
            StmtKind::While { .. } => todo!(),
            StmtKind::If { .. } => todo!(),
            StmtKind::With { .. } => todo!(),
            StmtKind::AsyncWith { .. } => todo!(),
            StmtKind::Match { .. } => todo!(),
            StmtKind::Raise { .. } => todo!(),
            StmtKind::Try { .. } => todo!(),
            StmtKind::Assert { .. } => todo!(),
            StmtKind::Import { .. } => todo!(),
            StmtKind::ImportFrom { .. } => todo!(),
            StmtKind::Global { .. } => todo!(),
            StmtKind::Nonlocal { .. } => todo!(),
            StmtKind::Expr { .. } => todo!(),
            StmtKind::Pass => todo!(),
            StmtKind::Break => todo!(),
            StmtKind::Continue => todo!(),
        }
    }
}
