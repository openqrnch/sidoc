use std::collections::HashMap;
use std::sync::Arc;

use crate::{Doc, Error, Node};

pub struct RenderContext {
  ichar: char,
  iwidth: usize,
  dict: HashMap<String, Arc<Doc>>
}


impl RenderContext {
  /// Create a new render context object.
  pub fn new() -> Self {
    RenderContext {
      ichar: ' ',
      iwidth: 2,
      dict: HashMap::new()
    }
  }

  /// Add a shared `Doc` document to the render context.
  pub fn doc<N: ToString>(&mut self, id: N, doc: Arc<Doc>) {
    self.dict.insert(id.to_string(), Arc::clone(&doc));
  }

  /// Render a root `Doc`, resolving all references.
  pub fn render(&self, name: &str) -> Result<String, Error> {
    struct IterNode<'a> {
      lst: &'a Vec<Node>,
      idx: usize
    }
    let mut iterstack = Vec::new();
    let mut out = String::new();
    let mut indent: usize = 0;

    // Generate single indent string
    let istr = self.ichar.to_string().repeat(self.iwidth);

    if let Some(dict) = self.dict.get(name) {
      iterstack.push(IterNode {
        lst: &dict.nodes,
        idx: 0
      });
    } else {
      return Err(Error::BadRef(format!("Missing root document '{}'", name)));
    }

    'outer: while !iterstack.is_empty() {
      let mut it = iterstack.pop().unwrap();

      while it.idx < it.lst.len() {
        match &it.lst[it.idx] {
          Node::BeginScope(s) => {
            let is = istr.repeat(indent);
            out.push_str(&is);
            out.push_str(&s);
            out.push('\n');
            indent += 1;
          }
          Node::EndScope(s) => {
            indent -= 1;
            if let Some(s) = s {
              let is = istr.repeat(indent);
              out.push_str(&is);
              out.push_str(&s);
              out.push('\n');
            }
          }
          Node::Line(s) => {
            let is = istr.repeat(indent);
            out.push_str(&is);
            out.push_str(&s);
            out.push('\n');
          }
          Node::OptRef(name) => {
            if let Some(dict) = self.dict.get(name) {
              iterstack.push(IterNode {
                lst: it.lst,
                idx: it.idx + 1
              });

              iterstack.push(IterNode {
                lst: &dict.nodes,
                idx: 0
              });
              continue 'outer;
            }
          }
          Node::ReqRef(name) => {
            if let Some(dict) = self.dict.get(name) {
              iterstack.push(IterNode {
                lst: it.lst,
                idx: it.idx + 1
              });

              iterstack.push(IterNode {
                lst: &dict.nodes,
                idx: 0
              });
              continue 'outer;
            } else {
              return Err(Error::BadRef(format!(
                "Missing required document '{}'",
                name
              )));
            }
          }
        }
        it.idx += 1;
      }
    }

    Ok(out)
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
