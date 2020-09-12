use crate::{Doc, Error, Node};

/// Constructor for `Doc` objects.
pub struct Builder {
  nodes: Vec<Node>,
  scope_stack: Vec<Option<String>>
}

impl Builder {
  /// Create a new `Doc` builder context.
  pub fn new() -> Self {
    Builder {
      nodes: Vec::new(),
      scope_stack: Vec::new()
    }
  }

  /// Begin a scope, pushing an optional scope terminator to the internal scope
  /// stack.
  ///
  /// If the scope generated using a terminator line, that line will appended
  /// to the document when the scope is closed using the `exit()` method.
  pub fn scope<L: ToString, K: ToString>(
    &mut self,
    begin_line: L,
    term_line: Option<K>
  ) -> &mut Self {
    self.nodes.push(Node::BeginScope(begin_line.to_string()));
    if let Some(ln) = term_line {
      self.scope_stack.push(Some(ln.to_string()));
    } else {
      self.scope_stack.push(None);
    }
    self
  }

  /// Leave a previously entered scope.
  ///
  /// If the `scope()` call that created the current scope
  pub fn exit(&mut self) -> &mut Self {
    if let Some(s) = self.scope_stack.pop().unwrap() {
      self.nodes.push(Node::EndScope(Some(s)));
    } else {
      self.nodes.push(Node::EndScope(None));
    }
    self
  }

  /// Leave previously entered scope, adding a line passed by the caller rather
  /// than the scope stack.
  pub fn exit_line<L: ToString>(&mut self, line: L) -> &mut Self {
    let _ = self.scope_stack.pop().unwrap();
    self.nodes.push(Node::EndScope(Some(line.to_string())));
    self
  }

  /// Add a new line at current scope.
  pub fn line<L: ToString>(&mut self, line: L) -> &mut Self {
    self.nodes.push(Node::Line(line.to_string()));
    self
  }

  /// Add a named optional reference.
  ///
  /// References are placeholders for other documents.  An optional reference
  /// means that this reference does not need to be resolved by the renderer.
  pub fn optref<N: ToString>(&mut self, name: N) -> &mut Self {
    self.nodes.push(Node::OptRef(name.to_string()));
    self
  }

  /// Add a named required reference.
  ///
  /// References are placeholders for other documents.  A required reference
  /// must be resolved by the renderer or it will return an error to its
  /// caller.
  pub fn reqref<N: ToString>(&mut self, name: N) -> &mut Self {
    self.nodes.push(Node::ReqRef(name.to_string()));
    self
  }

  /// Generate a `Doc` object from this document.
  ///
  /// The document must be properly nested before calling this function,
  /// meaning all scopes it opened must be closed.
  pub fn build(self) -> Result<Doc, Error> {
    if self.scope_stack.is_empty() {
      Ok(Doc { nodes: self.nodes })
    } else {
      Err(Error::BadNesting(format!(
        "{} scope(s) remaining",
        self.scope_stack.len()
      )))
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
