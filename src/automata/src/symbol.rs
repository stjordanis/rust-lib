//! Defines a Symbol that is operated on by the finite automata.

use crate::prelude::*;
use std::cmp::Ordering;


// =============
// === Types ===
// =============

/// The type of a symbol index.
pub type SymbolIndex = u64;



// ==============
// === Symbol ===
// ==============

/// An input symbol to a finite automaton.
#[derive(Clone,Debug)]
#[allow(missing_docs)]
pub struct Symbol {
    pub index : SymbolIndex,
    pub debug_name: String
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Symbol {
    fn cmp(&self, other:&Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}


impl Symbol {
    /// End of line symbol.
    pub fn eof() -> Self {
        Self::new(SymbolIndex::max_value())
    }

    /// Invalid symbol.
    pub fn invalid() -> Self {
        Self::new(SymbolIndex::max_value() - 1)
    }

    /// Null symbol.
    pub fn null() -> Self {
        Self::new(0)
    }

    /// Constructor.
    pub fn new(index:SymbolIndex) -> Self {
        let debug_name = "unnamed".into();
        Self{index,debug_name}
    }

    /// Named constructor.
    pub fn new_named(index:SymbolIndex, name:impl Into<String>) -> Self {
        let debug_name = name.into();
        Self{index,debug_name}
    }

    /// Next symbol, if any.
    pub fn next(&self) -> Option<Self> {
        self.index.checked_add(1).map(Self::new)
    }
}


// === Impls ===

impl Display for Symbol {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.debug_name)
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::null()
    }
}

impl From<u64> for Symbol {
    fn from(index:u64) -> Symbol {
        Symbol::new(index)
    }
}

impl From<char> for Symbol {
    fn from(ch:char) -> Symbol {
        Symbol::new_named(ch as SymbolIndex,format!("{}",ch))
    }
}

impl From<&Symbol> for Symbol {
    fn from(symbol:&Symbol) -> Self {
        symbol.clone()
    }
}
