//! FEEL context.

use crate::errors::*;
use crate::names::Name;
use crate::qualified_names::QualifiedName;
use crate::strings::ToFeelString;
use crate::value_null;
use crate::values::Value;
use dsntk_common::{DsntkError, Jsonify};
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

/// Type alias for context entries.
type FeelContextEntries = BTreeMap<Name, Value>;

/// The FEEL context.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeelContext(FeelContextEntries);

impl Deref for FeelContext {
  type Target = FeelContextEntries;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<Value> for FeelContext {
  type Error = DsntkError;
  /// Converts [Value] to [FeelContext].
  fn try_from(value: Value) -> Result<Self, Self::Error> {
    if let Value::Context(ctx) = value {
      Ok(ctx)
    } else {
      Err(err_value_is_not_a_context(&value))
    }
  }
}

impl From<FeelContext> for Value {
  /// Converts [FeelContext] to [Value].
  fn from(ctx: FeelContext) -> Self {
    Value::Context(ctx)
  }
}

impl fmt::Display for FeelContext {
  /// Convert [FeelContext] to string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{{{}}}",
      self.0.iter().map(|(name, value)| { format!(r#"{name}: {value}"#) }).collect::<Vec<String>>().join(", ")
    )
  }
}

impl ToFeelString for FeelContext {
  /// Converts [FeelContext] to FEEL string.
  fn to_feel_string(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| {
          let name_str = format!("{name}");
          let padded_name_str = match name_str.as_str() {
            "{" | "}" | ":" | "," => format!("\"{name_str}\""),
            "\"" => "\"\\\"\"".to_string(),
            _ => name_str,
          };
          format!(r#"{padded_name_str}: {value}"#)
        })
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl Jsonify for FeelContext {
  /// Converts [FeelContext] to JSON string.
  fn jsonify(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| format!(r#""{}": {}"#, name, value.jsonify()))
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl FeelContext {
  /// Returns `true` if context contains an entry specified by **name**.
  pub fn contains_entry(&self, name: &Name) -> bool {
    self.0.contains_key(name)
  }

  /// Returns `true` if this [FeelContext] contains an entry specified by [QualifiedName].
  pub fn contains_entries(&self, qname: &QualifiedName) -> bool {
    self.contains_deep(qname.as_slice())
  }

  /// Returns a value of an entry with specified name.
  pub fn get_entry(&self, name: &Name) -> Option<&Value> {
    self.0.get(name)
  }

  /// Sets a value for specified entry name.
  pub fn set_entry(&mut self, name: &Name, value: Value) {
    self.0.insert(name.clone(), value);
  }

  /// Removes a value of an entry with specified name.
  pub fn remove_entry(&mut self, name: &Name) -> Option<Value> {
    self.0.remove(name)
  }

  /// Sets a null value for specified entry.
  pub fn set_null(&mut self, name: Name) {
    self.0.insert(name, value_null!());
  }

  /// Returns a list of all [FeelContext] entries.
  pub fn get_entries(&self) -> Vec<(&Name, &Value)> {
    self.0.iter().collect::<Vec<(&Name, &Value)>>()
  }

  /// Returns an iterator over all entries in [FeelContext].
  pub fn iter(&self) -> Iter<Name, Value> {
    self.0.iter()
  }

  /// Returns a first value contained by context.
  pub fn get_first(&self) -> Option<&Value> {
    self.0.values().next()
  }

  /// Returns the number of entries in [FeelContext].
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns `true` if [FeelContext] is empty.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Returns `true` if [FeelContext] contains an entry with specified name, and the value is context.
  pub fn is_context(&self, name: &Name) -> bool {
    matches!(self.0.get(name), Some(Value::Context(_)))
  }

  ///
  pub fn zip(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      self.0.insert(name.clone(), value.clone());
    }
  }

  ///
  pub fn overwrite(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      if self.0.contains_key(name) {
        self.0.insert(name.clone(), value.clone());
      }
    }
  }

  //TODO refactor the name, this operation is not moving, it is like prefixing
  pub fn move_entry(&mut self, name: Name, parent: Name) {
    if let Some(value) = self.0.remove(&name) {
      self.create_entries(&[parent, name], value);
    }
  }

  /// Creates an entry with a value for specified [QualifiedName].
  /// All non existing intermediary contexts will be created.
  pub fn create_entry(&mut self, qname: &QualifiedName, value: Value) {
    self.create_entries(qname.as_slice(), value);
  }

  /// Searches for a value of an entry pointed by specified qualified name.
  pub fn search_entry<'a>(&'a self, qname: &'a QualifiedName) -> Option<&'a Value> {
    self.search_deep(qname.as_slice())
  }

  /// Deep check for a value pointed by slice of names.
  pub fn contains_deep(&self, names: &[Name]) -> bool {
    if names.is_empty() {
      return false;
    }
    let tail = &names[1..];
    if let Some(value) = self.0.get(&names[0]) {
      if tail.is_empty() {
        return true;
      }
      if let Value::Context(context) = value {
        return context.contains_deep(tail);
      }
    }
    false
  }

  /// Creates entries with intermediary contexts when needed.
  pub fn create_entries(&mut self, names: &[Name], value: Value) {
    // if there are no names, then return
    if names.is_empty() {
      return;
    }
    let key = names[0].clone();
    let tail = &names[1..];
    // if tail is empty, then insert the value under
    // specified key in current context and return
    if tail.is_empty() {
      self.0.insert(key, value);
      return;
    }
    // if there is a context under the specified key,
    // then insert value to this context and return
    if let Some(Value::Context(sub_ctx)) = self.0.get_mut(&key) {
      sub_ctx.create_entries(tail, value);
      return;
    }
    // finally, when got to this point, insert a value
    // to newly created context
    let mut sub_ctx = FeelContext::default();
    sub_ctx.create_entries(tail, value);
    self.0.insert(key, sub_ctx.into());
  }

  /// Deep search for a value pointed by names.
  pub fn search_deep(&self, names: &[Name]) -> Option<&Value> {
    if !names.is_empty() {
      let tail = &names[1..];
      if let Some(value) = self.0.get(&names[0]) {
        if let Value::Context(ctx) = value {
          return if tail.is_empty() { Some(value) } else { ctx.search_deep(tail) };
        } else if tail.is_empty() {
          return Some(value);
        }
      }
    }
    None
  }
}
