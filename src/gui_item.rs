use std::fmt::{Display};

use imgui::OptionOwned;
use serde::{Deserialize, Serialize};

use crate::{GuiProvider};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Item<T> {
  // elements
  Text(String),
  Button(String, Option<[f32; 2]>), // label, size
  CheckBox(String, bool),
  // control flow
  If(Box<(Item<bool>, Item<T>)>),
  IfElse(Box<(Item<bool>, Item<T>, Item<T>)>),
  Scope(Vec<Item<T>>),
  Return(T),
  _None,
}

impl Item<bool> {
  
  // elements
  pub fn button<D: Display>(label: D, size: impl OptionOwned<[f32; 2]>) -> Self {
    Self::Button(format!("{label}"), size.into_option())
  }
  pub fn checkbox<D: Display>(label: D, value: impl OptionOwned<bool>) -> Self {
    Self::CheckBox(format!("{label}"), value.into_option().unwrap_or(false))
  }

  pub fn draw_bool<P: GuiProvider>(&mut self, p: &mut P) -> bool {
    match self {
      Item::Button(label, size) => p.button(label, *size),
      Item::CheckBox(label, value) => p.checkbox(label, value),
      _ => panic!(),
    }
  }

}
impl<T: Clone> Item<T> {

  // elements
  pub fn text<D: Display>(text: D) -> Self {
    Self::Text(format!("{text}"))
  }

  // control
  pub fn let_<D: Display, U>(_name: D, _value: U) -> Self {
    unimplemented!()
  }
  pub fn if_(cond: Item<bool>, conseq: Item<T>) -> Self {
    Self::If(Box::new((cond, conseq)))
  }
  pub fn if_else(cond: Item<bool>, conseq: Item<T>, alt: Item<T>) -> Self {
    Self::IfElse(Box::new((cond, conseq, alt)))
  }
  pub fn scope<I: IntoIterator<Item = Item<T>>>(items: I) -> Self {
    Self::Scope(items.into_iter().collect())
  }
  pub fn return_(item: T) -> Self {
    Self::Return(item)
  }
  pub fn none() -> Self {
    Self::_None
  }

  pub fn draw<P: GuiProvider>(&mut self, p: &mut P) -> Option<T> {
    match self {
      // elements
      Item::Text(text) => {
        p.text(text);
        None
      },
      Item::Button(label, size) => {
        p.button(label, *size);
        None
      },
      Item::CheckBox(label, value) => {
        p.checkbox(label, value);
        None
      },
      // control flow
      Item::Scope(items) => {
        let mut res = None;
        for item in items {
          res = item.draw(p);
        }
        res
      },
      Item::If(items) => {
        let (cond, conseq) = &mut **items;
        if cond.draw_bool(p) {
          conseq.draw(p)
        } else {
          None
        }
      },
      Item::IfElse(items) => {
        let (cond, conseq, alt) = &mut **items;
        if cond.draw_bool(p) {
          conseq.draw(p)
        } else {
          alt.draw(p)
        }
      },
      Item::Return(t) => Some(t.clone()),
      Item::_None => None,
    }
  }

}




#[macro_export]
macro_rules! if_ {
  ($cond:expr, $conseq:expr $(,)?) => {
    if_!($cond, $conseq, Item::_None)
  };
  ($cond:expr, $conseq:expr, $alt:expr $(,)?) => {
    Item::IfElse(Box::new(($cond, $conseq, $alt)))
  };
}




#[cfg(test)]
pub mod gui_item {

  use crate::{provider::EmptyProvider};
  use super::Item::{ self, * };

  
  #[derive(Clone, Copy, PartialEq, Eq)]
  struct S(i32);


  #[test]
  fn test_if() {

    let s = S(123);

    let mut item = if_!(
      Button("+".into(), None),
      Return(s),
    );
    
    assert!(item.draw(&mut EmptyProvider::bool_true()).unwrap() == s);
    assert!(item.draw(&mut EmptyProvider::bool_false()).is_none());

  }
  fn test_scope() {

    //let s = S(0);
    //let mut item: Item<S> = Item::scope(vec![
    //  Item::let_("b", false),
    //  Item::checkbox("check", "b"),
    //  Item::if_(
    //    Item::var("b"),
    //    Item::text("checked"),
    //  )
    //]);


  }

}

unsafe fn _f() {

  let mut b = false;
  if imgui::checkbox("label", &mut b) {

  }

  if b {
    imgui::text("checked");
  }

}