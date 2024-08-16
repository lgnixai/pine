use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use crate::ast::nodeid::NodeId;

use crate::input::{Input, Span};

#[derive(Debug, Clone)]
pub struct ParserContext {
    // files: HashMap<PathBuf, SourceFile>,
    // diagnostics: Diagnostics,
    cur_file_path: PathBuf,
    identities: BTreeMap<NodeId, Span>,
    operators_list: HashMap<String, u8>,
    pub(crate) block_indent: usize,
    pub(crate) first_indent: Option<usize>,
    next_node_id: NodeId,
    pub(crate) source:String,
    //structs: HashMap<String, Type>,
    //pub config: Config,
    allow_newline_dot: Vec<()>,
}

impl ParserContext {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            source: "".parse().unwrap(),
            //files: HashMap::new(),
            cur_file_path: file_path,
            identities: BTreeMap::new(),
            operators_list: HashMap::new(),
            block_indent: 0,
            first_indent: None,
            next_node_id: 0,
            // structs: HashMap::new(),
            // diagnostics: Diagnostics::default(),
            // config,
            allow_newline_dot: vec![],
        }
    }
    //
    // #[cfg(test)]
    // pub fn new_with_operators(
    //     file_path: PathBuf,
    //     operators: HashMap<String, u8>,
    //     config: Config,
    // ) -> Self {
    //     Self {
    //         files: HashMap::new(),
    //         cur_file_path: file_path,
    //         identities: BTreeMap::new(),
    //         operators_list: operators,
    //         block_indent: 0,
    //         first_indent: None,
    //         next_node_id: 0,
    //         structs: HashMap::new(),
    //         diagnostics: Diagnostics::default(),
    //         config,
    //         allow_newline_dot: vec![],
    //     }
    // }
    //
    // pub fn new_from(&self, name: &str, config: Config) -> Self {
    //     Self {
    //         files: HashMap::new(),
    //         cur_file_path: self
    //             .cur_file_path
    //             .parent()
    //             .unwrap()
    //             .join(name.to_owned() + ".rk"),
    //         identities: BTreeMap::new(),
    //         operators_list: HashMap::new(),
    //         block_indent: 0,
    //         first_indent: None,
    //         next_node_id: self.next_node_id,
    //         structs: HashMap::new(),
    //         diagnostics: Diagnostics::default(), // FIXME
    //         config,
    //         allow_newline_dot: vec![],
    //     }
    // }
    //
    // pub fn new_std(&self, config: Config) -> Self {
    //     Self {
    //         files: HashMap::new(),
    //         cur_file_path: PathBuf::from("/std/src/lib.rk"),
    //         identities: BTreeMap::new(),
    //         operators_list: HashMap::new(),
    //         block_indent: 0,
    //         first_indent: None,
    //         next_node_id: self.next_node_id,
    //         structs: HashMap::new(),
    //         diagnostics: Diagnostics::default(),
    //         config,
    //         allow_newline_dot: vec![],
    //     }
    // }
    //
    pub fn new_identity(&mut self, span: Span) -> NodeId {
        let node_id = self.next_node_id;

        self.next_node_id += 1;

        self.identities.insert(node_id, span);

        node_id
    }

    pub fn current_file_path(&self) -> &PathBuf {
        &self.cur_file_path
    }

    pub fn operators(&self) -> &HashMap<String, u8> {
        &self.operators_list
    }

    pub fn add_operator(&mut self, op: String, prec: u8) {
        self.operators_list.insert(op, prec);
    }

    // pub fn identities(&self) -> BTreeMap<NodeId, Span> {
    //     self.identities.clone()
    // }

    pub fn operators_list(&self) -> HashMap<String, u8> {
        self.operators_list.clone()
    }

    // pub fn files(&self) -> HashMap<PathBuf, SourceFile> {
    //     self.files.clone()
    // }

   // pub fn diagnostics(&self) -> Diagnostics {
   //      self.diagnostics.clone()
   //  }
}
