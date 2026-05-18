use std::collections::HashMap;

use crate::ast::types::NexisType;

#[derive(Debug, Clone)]
pub struct Symbol {

    pub name: String,

    pub datatype: NexisType,

    pub mutable: bool,
}

#[derive(Debug)]
pub struct SymbolTable {

    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {

    pub fn new() -> Self {

        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {

        self.symbols.insert(
            symbol.name.clone(),
            symbol
        );
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {

        self.symbols.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {

        self.symbols.contains_key(name)
    }
}