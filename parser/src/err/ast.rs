use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, Hash)]
pub struct Location {
    pub offset: usize,
    pub line: u32,
    pub column: usize,
}


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Definition {
    Lines {
        count: usize,
    },
    Comment {
        text: String,
    },

}
