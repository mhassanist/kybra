use cdk_framework::act::node::candid;
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{errors::CollectResults, py_ast::PyAst, source_map::SourceMapped, Error};

struct TypeAlias<'a> {
    enclosed_expr: &'a Located<ExprKind>,
}

impl PyAst {
    pub fn build_type_aliases(&self) -> Result<Vec<candid::TypeAlias>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_type_alias())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn get_type_alias(&self) -> Result<Option<TypeAlias>, Error> {
        if let StmtKind::Assign { value, .. }
        | StmtKind::AnnAssign {
            value: Some(value), ..
        } = &self.node
        {
            if let ExprKind::Subscript { value, slice, .. } = &value.node {
                if let ExprKind::Name { id, .. } = &value.node {
                    match id == "Alias" {
                        true => {
                            return Ok(Some(TypeAlias {
                                enclosed_expr: slice,
                            }));
                        }
                        false => return Ok(None),
                    }
                }
            }
        }
        Ok(None)
    }

    fn as_type_alias(&self) -> Result<Option<candid::TypeAlias>, Vec<Error>> {
        let type_alias = match self.get_type_alias().map_err(Into::<Vec<Error>>::into)? {
            Some(type_alias) => type_alias,
            None => return Ok(None),
        };

        let name = self.get_name_or_err()?;

        let enclosed_type = SourceMapped::new(type_alias.enclosed_expr, self.source_map.clone())
            .to_candid_type()?;
        Ok(Some(candid::TypeAlias {
            name,
            aliased_type: Box::new(enclosed_type),
            type_params: vec![].into(),
        }))
    }
}
