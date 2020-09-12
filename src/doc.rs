use crate::Node;

/// A "Doc" represents a set of lines and references to other Doc's.
pub struct Doc {
  pub(crate) nodes: Vec<Node>
}

impl Doc {
  pub fn new() -> Self {
    Doc { nodes: Vec::new() }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
