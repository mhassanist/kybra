pub mod errors;
pub mod rust;

use crate::{
    errors::{CollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};
use cdk_framework::{act::node::CandidType, traits::CollectResults as OtherCollectResults};
use num_bigint::{BigInt, Sign};
use rustpython_parser::ast::{Constant, ExprKind, KeywordData, Located, StmtKind};

use self::errors::{
    InvalidMemoryId, MaxKeySizeMissing, MaxSizeMustBeInteger, MaxSizeMustBeNonNegative,
    MaxSizeTooBig, MaxValueSizeMissing, MemoryIdMustBeAnInteger, MemoryIdMustBeInteger,
    MemoryIdMustBeNonNegative, MemoryIdTooBig, MissingMemoryId, StableBTreeMapNodeFormat,
};

// TODO all variables should be called stable_b_tree_map_nodes
#[derive(Clone)]
pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: CandidType,
    pub value_type: CandidType,
    pub max_key_size: u32,
    pub max_value_size: u32,
}

impl PyAst {
    pub fn build_stable_b_tree_map_nodes(&self) -> Result<Vec<StableBTreeMapNode>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_stable_b_tree_map_node())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<ExprKind>> {
    fn is_stable_b_tree_map_node(&self) -> bool {
        match &self.node {
            ExprKind::Call { func, .. } => match &func.node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => id == "StableBTreeMap",
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn get_value_type(&self) -> Result<SourceMapped<&Located<ExprKind>>, Error> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[1], self.source_map.clone()))
                }
                _ => Err(StableBTreeMapNodeFormat::err_from_expr(self)),
            },
            _ => Err(Unreachable::error()),
        }
    }

    fn get_key_type(&self) -> Result<SourceMapped<&Located<ExprKind>>, Error> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[0], self.source_map.clone()))
                }
                _ => Err(StableBTreeMapNodeFormat::err_from_expr(self).into()),
            },
            _ => Err(Unreachable::error()),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn is_stable_b_tree_map_node(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => SourceMapped::new(value.as_ref(), self.source_map.clone())
                .is_stable_b_tree_map_node(),
            _ => false,
        }
    }

    fn as_stable_b_tree_map_node(&self) -> Result<Option<StableBTreeMapNode>, Vec<Error>> {
        if !self.is_stable_b_tree_map_node() {
            return Ok(None);
        }
        let (memory_id, key_type, value_type, max_key_size, max_value_size) = (
            self.get_memory_id().map_err(Error::into),
            self.get_key_type(),
            self.get_value_type(),
            self.get_max_key_size().map_err(Error::into),
            self.get_max_value_size().map_err(Error::into),
        )
            .collect_results()?;
        Ok(Some(StableBTreeMapNode {
            memory_id,
            key_type,
            value_type,
            max_key_size,
            max_value_size,
        }))
    }

    fn get_assign_value(&self) -> Option<&Located<ExprKind>> {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => Some(value),
            _ => None,
        }
    }

    fn get_memory_id(&self) -> Result<u8, Error> {
        let assign_value = match self.get_assign_value() {
            Some(assign_value) => assign_value,
            None => return Err(Unreachable::error()),
        };
        match &assign_value.node {
            ExprKind::Call { args, keywords, .. } => {
                // Try to get it from args
                if args.len() >= 1 {
                    return match &args[0].node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(integer) => self.big_int_to_memory_id(integer),
                            _ => Err(MemoryIdMustBeAnInteger::err_from_stmt(self)),
                        },
                        _ => Err(InvalidMemoryId::err_from_stmt(self)),
                    };
                }
                // Try to get it from keywords
                if let Some(memory_id) = self.get_memory_id_from_keywords(keywords)? {
                    return Ok(memory_id);
                }
                // It was in neither the keywords nor the args
                Err(MissingMemoryId::err_from_stmt(self))
            }
            _ => Err(Unreachable::error()),
        }
    }

    fn get_key_type(&self) -> Result<CandidType, Vec<Error>> {
        let assign_value = match self.get_assign_value() {
            Some(assign_value) => assign_value,
            None => return Err(Unreachable::error().into()),
        };
        match &assign_value.node {
            ExprKind::Call { func, .. } => {
                SourceMapped::new(func.as_ref(), self.source_map.clone())
                    .get_key_type()
                    .map_err(Into::<Vec<Error>>::into)?
                    .to_candid_type()
            }
            _ => Err(Unreachable::error().into()),
        }
    }

    fn get_value_type(&self) -> Result<CandidType, Vec<Error>> {
        let assign_value = match self.get_assign_value() {
            Some(assign_value) => assign_value,
            None => return Err(Unreachable::error().into()),
        };
        match &assign_value.node {
            ExprKind::Call { func, .. } => {
                SourceMapped::new(func.as_ref(), self.source_map.clone())
                    .get_value_type()
                    .map_err(Into::<Vec<Error>>::into)?
                    .to_candid_type()
            }
            _ => Err(Unreachable::error().into()),
        }
    }

    fn get_max_key_size(&self) -> Result<u32, Error> {
        let assign_value = match self.get_assign_value() {
            Some(assign_value) => assign_value,
            None => return Err(Unreachable::error().into()),
        };
        match &assign_value.node {
            ExprKind::Call { args, keywords, .. } => {
                // Try to get it from args
                if args.len() >= 2 {
                    return self.get_max_size_from_args(1, args);
                }
                // Try to get it from keywords
                if let Some(max_key_size) = self.get_max_size_from_keywords("key", keywords)? {
                    return Ok(max_key_size);
                }
                // It was in neither the keywords nor the args
                Err(MaxKeySizeMissing::err_from_stmt(self))
            }
            _ => Err(Unreachable::error()),
        }
    }

    fn get_max_value_size(&self) -> Result<u32, Error> {
        let assign_value = match self.get_assign_value() {
            Some(assign_value) => assign_value,
            None => return Err(Unreachable::error().into()),
        };
        match &assign_value.node {
            ExprKind::Call { args, keywords, .. } => {
                // Try to get it from args
                if args.len() >= 3 {
                    return self.get_max_size_from_args(2, args);
                }
                // Try to get it from keywords
                if let Some(max_key_size) = self.get_max_size_from_keywords("value", keywords)? {
                    return Ok(max_key_size);
                }
                // It was in neither the keywords nor the args
                Err(MaxValueSizeMissing::err_from_stmt(self))
            }
            _ => Err(Unreachable::error()),
        }
    }

    // Helper method for get_max_key_size and get_max_value_size
    fn get_max_size_from_keywords(
        &self,
        name: &str,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Result<Option<u32>, Error> {
        match get_keyword_by_name(format!("max_{}_size", name).as_str(), keywords) {
            Some(keyword) => match &keyword.node.value.node {
                ExprKind::Constant { value, .. } => match value {
                    Constant::Int(int) => Ok(Some(self.big_int_to_max_size(int)?)),
                    _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
                },
                _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
            },
            None => Ok(None),
        }
    }

    fn get_max_size_from_args(
        &self,
        name: usize,
        keywords: &Vec<Located<ExprKind>>,
    ) -> Result<u32, Error> {
        match &keywords[name].node {
            ExprKind::Constant { value, .. } => match value {
                Constant::Int(integer) => self.big_int_to_max_size(integer),
                _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
            },
            _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
        }
    }

    // Helper method for get_memory_id
    fn get_memory_id_from_keywords(
        &self,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Result<Option<u8>, Error> {
        match get_keyword_by_name("memory_id", keywords) {
            Some(keyword) => {
                let memory_id = match &keyword.node.value.node {
                    ExprKind::Constant { value, .. } => match value {
                        Constant::Int(int) => self.big_int_to_memory_id(int)?,
                        _ => return Err(MemoryIdMustBeInteger::err_from_stmt(self)),
                    },
                    _ => return Err(MemoryIdMustBeInteger::err_from_stmt(self)),
                };
                Ok(Some(memory_id))
            }
            None => return Ok(None),
        }
    }

    fn big_int_to_max_size(&self, num: &BigInt) -> Result<u32, Error> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(MaxSizeMustBeNonNegative::err_from_stmt(self));
        }
        if digits.1.len() > 1 {
            return Err(MaxSizeTooBig::err_from_stmt(self));
        }
        Ok(digits.1[0])
    }

    fn big_int_to_memory_id(&self, num: &BigInt) -> Result<u8, Error> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(MemoryIdMustBeNonNegative::err_from_stmt(self));
        }
        if digits.1.len() > 1 {
            return Err(MemoryIdTooBig::err_from_stmt(self));
        }
        if digits.1.len() == 0 {
            return Ok(0);
        }
        let value = digits.1[0];
        if value > u8::MAX as u32 {
            return Err(MemoryIdTooBig::err_from_stmt(self));
        }
        Ok(value as u8)
    }
}

fn get_keyword_by_name<'a>(
    name: &str,
    keywords: &'a Vec<Located<KeywordData>>,
) -> Option<&'a Located<KeywordData>> {
    keywords
        .iter()
        .find(|keyword| keyword.node.arg.as_deref() == Some(name))
}
