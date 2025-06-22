
use std::fmt::Display;
use imgui::ImGuiTableColumnFlags_NoHeaderLabel;

use crate::{provider::Provider, GuiCallback, GuiDisplay, GuiFlags, GuiProvider};


pub struct Window {
  content: Vec<Box<dyn FnMut(&mut Provider)>>,
}
impl Window {
  pub fn new() -> Self {
    Window { 
      content: Vec::new(), 
    }
  }
  fn push_content(&mut self, content: impl FnMut(&mut Provider) + 'static) {
    self.content.push(Box::new(content));
  }
}

impl GuiProvider for Window {

  fn set_cursor_pos(&mut self, x: f32, y: f32) {
    self.push_content(move |p| p.set_cursor_pos(x, y));
  }
  
  fn begin<D: GuiDisplay>(&mut self, _: &D, _: Option<&mut bool>, _: impl GuiFlags) {

  }
  fn end(&mut self) {

  }
  
  fn text<D: GuiDisplay>(&mut self, text: &D) {
    let text = text.clone();
    self.push_content(move |p| p.text(&text));
  }
  fn button<D: GuiDisplay>(&mut self, label: &D, size: Option<[f32; 2]>) -> bool {
    let label = label.clone();
    self.push_content(
      move |p| { p.button(&label, size); }
    );
    panic!()
  }
  
}

