use std::collections::HashMap;

pub struct SymbolTable {
    ram: HashMap<String, u16>,
    var_address: u16,
    rom: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut ram: HashMap<String, u16> = HashMap::new();

        // Define pre-defined symbols
        ram.insert(String::from("SP"), 0x0);
        ram.insert(String::from("LCL"), 0x1);
        ram.insert(String::from("ARG"), 0x2);
        ram.insert(String::from("THIS"), 0x3);
        ram.insert(String::from("THAT"), 0x4);
        ram.insert(String::from("R0"), 0x0);
        ram.insert(String::from("R1"), 0x1);
        ram.insert(String::from("R2"), 0x2);
        ram.insert(String::from("R3"), 0x3);
        ram.insert(String::from("R4"), 0x4);
        ram.insert(String::from("R5"), 0x5);
        ram.insert(String::from("R6"), 0x6);
        ram.insert(String::from("R7"), 0x7);
        ram.insert(String::from("R8"), 0x8);
        ram.insert(String::from("R9"), 0x9);
        ram.insert(String::from("R10"), 0xA);
        ram.insert(String::from("R11"), 0xB);
        ram.insert(String::from("R12"), 0xC);
        ram.insert(String::from("R13"), 0xD);
        ram.insert(String::from("R14"), 0xE);
        ram.insert(String::from("R15"), 0xF);
        ram.insert(String::from("SCREEN"), 0x4000);
        ram.insert(String::from("KBD"), 0x6000);

        SymbolTable {
            ram: ram,
            var_address: 0x10,
            rom: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: &str, address: u16) {
        if self.rom.get(symbol).is_none() {
            self.rom.insert(symbol.to_string(), address);
        }
    }

    pub fn address_for(&mut self, symbol: &str) -> u16 {
        let r = &mut self.ram;
        let va = &mut self.var_address;
        self.rom.get(symbol).unwrap_or_else(|| {
            &*r.entry(symbol.to_string()).or_insert_with(|| {
                let address = *va;
                *va += 1;
                address
            })
        }).clone()
    }
}