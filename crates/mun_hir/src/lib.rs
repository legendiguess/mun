//! HIR provides high-level, object-oriented access to Mun code. It is constructed by first parsing
//! Mun code with the mun_syntax crate and then it is lowered into HIR constructs, names are
//! resolved, and type checking is performed. HIR is the input for both the compiler as well as the
//! language server.

#![allow(dead_code)]

#[macro_use]
mod macros;
#[macro_use]
mod arena;
mod adt;
mod builtin_type;
mod code_model;
mod db;
pub mod diagnostics;
mod display;
mod expr;
mod fixture;
mod ids;
mod in_file;
mod input;
mod item_tree;
pub mod line_index;
mod model;
mod name;
mod name_resolution;
mod path;
mod resolve;
mod source_id;
mod ty;
mod type_ref;
mod utils;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use salsa;

pub use relative_path::{RelativePath, RelativePathBuf};

pub use crate::{
    builtin_type::{FloatBitness, IntBitness, Signedness},
    db::{
        AstDatabase, AstDatabaseStorage, DefDatabase, DefDatabaseStorage, HirDatabase,
        HirDatabaseStorage, InternDatabase, InternDatabaseStorage, SourceDatabase,
        SourceDatabaseStorage, Upcast,
    },
    diagnostics::{Diagnostic, DiagnosticSink},
    display::HirDisplay,
    expr::{
        resolver_for_expr, ArithOp, BinaryOp, Body, CmpOp, Expr, ExprId, ExprScopes, Literal,
        LogicOp, Ordering, Pat, PatId, RecordLitField, Statement, UnaryOp,
    },
    ids::ItemLoc,
    in_file::InFile,
    input::{FileId, SourceRoot, SourceRootId},
    name::Name,
    name_resolution::PerNs,
    path::{Path, PathKind},
    resolve::{Resolution, Resolver},
    ty::{
        lower::CallableDef, ApplicationTy, FloatTy, InferenceResult, IntTy, ResolveBitness, Ty,
        TypeCtor,
    },
};

use crate::{name::AsName, source_id::AstIdMap};

pub use self::adt::StructMemoryKind;
pub use self::code_model::{
    Function, FunctionData, Module, ModuleDef, Struct, TypeAlias, Visibility,
};
