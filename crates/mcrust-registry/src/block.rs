use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId(pub u32);

impl BlockId {
    pub const AIR: BlockId = BlockId(0);
    pub const STONE: BlockId = BlockId(1);
}

#[derive(Debug, Clone, Default)]
pub struct BlockRegistry {
    java_names: Vec<&'static str>,
}

impl BlockRegistry {
    pub fn vanilla_minimal() -> Self {
        Self {
            java_names: vec!["minecraft:air", "minecraft:stone"],
        }
    }

    pub fn java_name(&self, id: BlockId) -> Option<&'static str> {
        self.java_names.get(id.0 as usize).copied()
    }
}
