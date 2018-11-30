use std::collections::HashMap;

pub struct SymbolTable {
    symbols: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: &str, address: u16) {
        if self.address_for(symbol) == None {
            self.symbols.insert(symbol, address);
        }
    }

    pub fn address_for(&self, symbol: &str) -> Option<&u16> {
        match symbol {
            "SP" => Some(&0),
            "LCL" => Some(&1),
            "ARG" => Some(&2),
            "THIS" => Some(&3),
            "THAT" => Some(&4),
            "R0" => Some(&0),
            "R1" => Some(&1),
            "R2" => Some(&2),
            "R3" => Some(&3),
            "R4" => Some(&4),
            "R5" => Some(&5),
            "R6" => Some(&6),
            "R7" => Some(&7),
            "R8" => Some(&8),
            "R9" => Some(&9),
            "R10" => Some(&10),
            "R11" => Some(&11),
            "R12" => Some(&12),
            "R13" => Some(&13),
            "R14" => Some(&14),
            "R15" => Some(&15),
            "SCREEN" => Some(&16384),
            "KBD" => Some(&24576),
            _ => self.symbols.get(symbol),  
        }
    }
}