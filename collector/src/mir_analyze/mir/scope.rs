use super::{mir::LocalID, ty::Ty};

pub type Scopes = Vec<Scope>;

#[derive(Debug)]
pub struct Scope {
    pub scope_id: ScopeID,
    pub inlined_func: Option<String>,
    pub var_debug_infos: VarDebugInfos,
    pub local_defs: LocalDefs,
    pub sub_scopes: Scopes,
}

pub type ScopeID = u32;

pub type VarDebugInfos = Vec<VarDebugInfo>;

#[derive(Debug)]
pub struct VarDebugInfo {
    pub name: String,
    pub local_id: LocalID,
}

pub type LocalDefs = Vec<LocalDef>;

#[derive(Debug)]
pub struct LocalDef {
    pub local_id: LocalID,
    pub ty: Ty,
    pub mutability: bool,
}
