use std::str::FromStr;

use super::mir::ModuledIdentifier;

#[derive(Debug)]
pub enum Ty {
    Unit,
    Bool,
    I32,
    Str,
    SelfDef(ModuledIdentifier),
    Tuple(Vec<Ty>),
    Array(Array),
    Ref(Box<Ty>),
    UND,
}

impl Default for Ty {
    fn default() -> Self {
        Self::UND
    }
}

impl FromStr for Ty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().nth(0) {
            if c == '&' {
                return Ok(Self::Ref(Box::new(Self::from_str(&s[1..])?)));
            } else if c == '[' {
                return Ok(Self::Array(Array::from_str(s)?));
            }
        }

        match s {
            "()" => Ok(Self::Unit),
            "i32" => Ok(Self::I32),
            "bool" => Ok(Self::Bool),
            "undef" => Ok(Self::UND),
            "str" => Ok(Self::Str),
            _ => Ok(Self::SelfDef(
                s.split("::").map(|s| s.to_string()).collect(),
            )),
        }
    }
}

impl From<ModuledIdentifier> for Ty {
    fn from(value: ModuledIdentifier) -> Self {
        Self::SelfDef(value)
    }
}

#[derive(Debug)]
pub struct Array {
    pub elem_ty: Box<Ty>,
    pub len: Option<u32>,
}

impl FromStr for Array {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s[1..s.len() - 1].split(';').map(|s| s).collect();
        Ok(Self {
            elem_ty: Box::new(Ty::from_str(s.get(0).unwrap())?),
            len: if let Some(s) = s.get(1) {
                Some(u32::from_str(s).unwrap())
            } else {
                None
            },
        })
    }
}
