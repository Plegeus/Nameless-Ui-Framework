
use crate::GuiProvider;


#[derive(Default)]
pub struct EmptyProvider {
  default_bool: bool,
  default_size: (f32, f32),
}
impl EmptyProvider {
  pub fn bool_true() -> Self {
    let mut p = Self::default();
    p.default_bool = true;
    p
  }
  pub fn bool_false() -> Self {
    let mut p = Self::default();
    p.default_bool = false;
    p
  }
}

impl GuiProvider for EmptyProvider {
  fn push_style_color(&mut self, _: imgui::ColorVar, _: impl Into<imgui::Color>) {
    
  }
  fn pop_style_color(&mut self, _: usize) {
    
  }
  fn push_style_var(&mut self, _: imgui::StyleVar, _: f32, _: impl imgui::OptionOwned<f32>) {
    
  }
  fn pop_style_var(&mut self, _: usize) {
    
  }
  fn get_content_region_avail(&mut self) -> (f32, f32) {
    self.default_size
  }
  fn is_item_clicked(&mut self, _: impl imgui::OptionOwned<imgui::MouseButton>) -> bool {
    self.default_bool
  }
  fn set_cursor_pos(&mut self, _: f32, _: f32) {
    
  }
  fn set_next_item_width(&mut self, _: f32) {
    
  }
  fn same_line(&mut self, _: impl imgui::OptionOwned<f32>, _: impl imgui::OptionOwned<f32>) {
    
  }
  fn push_item_width(&mut self, _: f32) {
    
  }
  fn set_next_window_pos(&mut self, _: [f32; 2], _: impl imgui::OptionOwned<imgui::Cond>, _: impl imgui::OptionOwned<[f32; 2]>) {
    
  }
  fn set_next_window_size(&mut self, _: [f32; 2], _: impl imgui::OptionOwned<imgui::Cond>) {
    
  }
  fn get_window_pos(&mut self) -> (f32, f32) {
    self.default_size
  }
  fn begin<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionMut<'a, bool>, _: impl imgui::OptionRef<'a, imgui::WindowFlags>) {
    
  }
  fn end(&mut self) {
    
  }
  fn begin_child<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionOwned<[f32; 2]>, _: impl imgui::OptionRef<'a, imgui::ChildFlags>, _: impl imgui::OptionRef<'a, imgui::WindowFlags>) {
    
  }
  fn end_child(&mut self) {
    
  }
  fn open_popup<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionRef<'a, imgui::PopupFlags>) {
    
  }
  fn begin_popup<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionRef<'a, imgui::WindowFlags>) -> bool {
    self.default_bool
  }
  fn begin_popup_modal<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionMut<'a, bool>, _: impl imgui::OptionRef<'a, imgui::WindowFlags>) -> bool {
    self.default_bool
  }
  fn end_popup(&mut self) {
    
  }
  fn push_id(&mut self, _: i32) {
    
  }
  fn pop_id(&mut self) {
    
  }
  fn separator(&mut self) {
    
  }
  fn separator_text<D: std::fmt::Display>(&mut self, _: D) {
    
  }
  fn columns<D: std::fmt::Display + 'static>(&mut self, _: i32, _: impl imgui::OptionOwned<D>, _: impl imgui::OptionOwned<bool>) {
    
  }
  fn table_setup_column<'a, D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionRef<'a, imgui::TableColumnFlags>, _: impl imgui::OptionOwned<f32>, _: impl imgui::OptionOwned<u32>) {
    
  }
  fn table_headers_row(&mut self) {
    
  }
  fn begin_table<'a, D: std::fmt::Display>(&mut self, _: D, _: i32, _: impl imgui::OptionRef<'a, imgui::TableFlags>) -> bool {
    self.default_bool
  }
  fn table_set_column_index(&mut self, _: i32) -> bool {
    self.default_bool
  }
  fn table_next_row<'a>(&mut self, _: impl imgui::OptionRef<'a, imgui::TableRowFlags>, _: impl imgui::OptionOwned<f32>) {
    
  }
  fn table_next_column(&mut self) -> bool {
    self.default_bool
  }
  fn end_table(&mut self) {
    
  }
  fn text<D: std::fmt::Display>(&mut self, _: D) {
    
  }
  fn text_wrapped<D: std::fmt::Display>(&mut self, _: D) {
    
  }
  fn button<D: std::fmt::Display>(&mut self, _: D, _: impl imgui::OptionOwned<[f32; 2]>) -> bool {
    self.default_bool
  }
  fn checkbox<D: std::fmt::Display>(&mut self, _: D, _: &mut bool) -> bool {
    self.default_bool
  }
  fn input_text<'a, D: std::fmt::Display>(&mut self, _: D, _: &mut String, _: impl imgui::OptionRef<'a, imgui::InputTextFlags>, _: impl imgui::OptionOwned<fn()>, _: impl imgui::OptionOwned<*const ()>) -> bool {
    self.default_bool
  }
  fn input_text_multi_line<'a, D: std::fmt::Display>(&mut self, _: D, _: &mut String, _: impl imgui::OptionOwned<[f32; 2]>, _: impl imgui::OptionRef<'a, imgui::InputTextFlags>, _: impl imgui::OptionOwned<fn()>, _: impl imgui::OptionOwned<*const ()>) -> bool {
    self.default_bool
  }
}






