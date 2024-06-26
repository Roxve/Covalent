use std::collections::HashMap;

use crate::parser::ast::{Blueprint, Literal};
use crate::types::{self, AtomDetails, AtomKind, AtomType, BasicType, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub ty: AtomType,

    pub value: Option<Literal>,

    pub expected: Option<AtomType>,
}

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Enviroment>>,
    pub blueprints: Vec<Blueprint>,
}

impl Enviroment {
    // Initialize the top-level environment. This environment serves as the parent for all other environments and contains the built-in types.
    pub fn init() -> Self {
        let mut symbols = HashMap::new();

        macro_rules! insert {
            ($name: expr, $type: expr) => {
                symbols.insert(
                    $name.to_owned(),
                    Symbol {
                        name: $name.to_owned(),
                        ty: $type,
                        value: None,
                        expected: None,
                    },
                );
            };
        }

        macro_rules! ty {
            ($type: expr) => {
                let name = $type.clone().to_string();
                insert!(
                    &name,
                    AtomType {
                        kind: $type.clone(),
                        details: Some(AtomDetails::Type)
                    }
                );
            };
        }

        macro_rules! complex {
            ($atom: expr) => {
                insert!(
                    &$atom.name,
                    AtomType {
                        kind: AtomKind::Atom($atom.clone()),
                        details: Some(AtomDetails::Type)
                    }
                );
            };
        }

        // default built-in types
        ty!(AtomKind::Basic(BasicType::Int));
        ty!(AtomKind::Basic(BasicType::Float));
        ty!(AtomKind::Basic(BasicType::Void));
        ty!(AtomKind::Dynamic);
        ty!(AtomKind::Basic(BasicType::Bool));

        // complex built-in types
        complex!(types::List);
        complex!(types::Back);
        complex!(types::Str);
        complex!(types::Const);

        Self {
            symbols,
            parent: None,
            blueprints: Vec::new(),
        }
    }

    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
            blueprints: Vec::new(),
        }
    }

    pub fn child(&mut self) {
        *self = Enviroment::new(Some(Box::new(self.clone())));
    }

    pub fn parent(&mut self) {
        *self = *self.parent.as_ref().unwrap().clone();
    }

    pub fn get_ty(&self, name: &String) -> Option<AtomType> {
        let sym = self.get(name);

        if sym.is_some() {
            Some(sym.unwrap().ty.clone())
        } else {
            if self.parent.is_some() {
                self.parent.as_ref().unwrap().get_ty(name)
            } else {
                None
            }
        }
    }

    pub fn get(&self, name: &String) -> Option<&Symbol> {
        if self.symbols.contains_key(name) {
            return Some(&self.symbols[name]);
        }

        if self.parent.is_some() {
            self.parent.as_ref().unwrap().get(name)
        } else {
            None
        }
    }

    pub fn has(&self, name: &String) -> bool {
        if self.symbols.contains_key(name) {
            true
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().has(name)
        } else {
            false
        }
    }

    //TODO: REMOVE
    // member expr parent is passed to a function as first arg if it takes it as an arg for ex.
    // set push: List(T) self, T item -> List(T)
    pub fn ty_parent_fn(&self, ty: &AtomType, name: &String) -> Option<AtomType> {
        let parent = self.symbols.get(name);

        if parent.is_some() {
            if let &AtomKind::Function(ref f) = &parent.unwrap().ty.kind {
                if &f.params[0] == ty {
                    return Some(parent.unwrap().to_owned().ty);
                }
            }
        }
        None
    }

    pub fn modify_ty(&mut self, name: &String, ty: AtomType) {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name).map(|val| val.ty = ty);
        } else if self.parent.is_some() {
            self.parent.as_mut().unwrap().modify_ty(name, ty);
        }
    }

    pub fn modify(&mut self, name: &String, sym: Symbol) {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name).map(|val| *val = sym);
        } else if self.parent.is_some() {
            self.parent.as_mut().unwrap().modify(name, sym);
        }
    }

    pub fn add(&mut self, sym: Symbol) {
        let name = sym.name.clone();
        if self.symbols.contains_key(&name) {
            self.modify(&name, sym)
        } else {
            self.symbols.insert(name, sym);
        }
    }

    pub fn get_blueprint(&self, name: &String) -> Option<Blueprint> {
        for blueprint in &self.blueprints {
            if blueprint.name.val() == name {
                return Some(blueprint.clone());
            }
        }

        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get_blueprint(name);
        }
        return None;
    }

    pub fn top(&mut self) -> &mut Enviroment {
        if self.parent.is_none() {
            self
        } else {
            self.parent.as_mut().unwrap().top()
        }
    } // returns the top level enviroment

    pub fn push_function(&mut self, name: String, func: FunctionType) {
        self.add(Symbol {
            name,
            ty: AtomType {
                kind: AtomKind::Function(func),
                details: None,
            },

            value: None,
            expected: None,
        });
    }

    pub fn expect(&mut self, name: &String, ty: AtomType) {
        self.symbols
            .get_mut(name)
            .map(|sym| sym.expected = Some(ty));
    }

    pub fn is_expected(&mut self, name: &String, ty: &AtomType) -> bool {
        let sym = self.get(name);
        if sym.is_none() {
            if self.parent.is_some() {
                return self.parent.as_mut().unwrap().is_expected(name, ty);
            }

            panic!("symbol not found");
        }

        if sym.unwrap().expected.is_none() {
            return true;
        }

        &sym.unwrap().expected == &Some(ty.clone())
    }
}
