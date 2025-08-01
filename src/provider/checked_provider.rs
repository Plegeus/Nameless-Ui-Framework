
use std::{fmt::Display};

use imgui::*;
use imgui_derive::assert_imgui_context;

use crate::GuiProvider;


pub struct CheckedProvider {

}
impl CheckedProvider {
  pub fn new() -> Self {
    CheckedProvider {  }
  }
}


impl GuiProvider for CheckedProvider {

  #[assert_imgui_context]
  fn push_style_color(&mut self, idx: ColorVar, color: impl Into<Color>) {
    unsafe {
      imgui::push_style_color(idx, color);
    }
  }
  #[assert_imgui_context]
  fn pop_style_color(&mut self, n: usize) {
    unsafe {
      imgui::pop_style_color(n as i32);
    }
  }
  #[assert_imgui_context]
  fn push_style_var(&mut self, idx: StyleVar, x: f32, y: impl OptionOwned<f32>) {
    unsafe {
      imgui::push_style_var(idx, x, y);
    } 
  }
  #[assert_imgui_context]
  fn pop_style_var(&mut self, n: usize) {
    unsafe {
      imgui::pop_style_var(n);
    }
  }

  #[assert_imgui_context]
  fn get_content_region_avail(&mut self) -> (f32, f32) {
    unsafe {
      imgui::get_content_region_avail()
    }
  }
  #[assert_imgui_context]
  fn is_item_clicked(&mut self, mouse_button: impl OptionOwned<MouseButton>) -> bool {
    unsafe {
      imgui::is_item_clicked(mouse_button)
    }
  }
  
  #[assert_imgui_context]
  fn set_cursor_pos(&mut self, x: f32, y: f32) {
    unsafe {
      imgui::set_cursor_pos(x, y);
    }
  }
  #[assert_imgui_context]
  fn set_next_item_width(&mut self, w: f32) {
    unsafe {
      imgui::set_next_item_width(w);
    }
  }
  #[assert_imgui_context]
  fn same_line(&mut self, offset: impl OptionOwned<f32>, spacing: impl OptionOwned<f32>) {
    unsafe {
      imgui::same_line(offset, spacing);
    }
  }
  #[assert_imgui_context]
  fn push_item_width(&mut self, w: f32) {
    unsafe {
      imgui::push_item_width(w);
    }
  }
  #[assert_imgui_context]
  fn set_next_window_pos(&mut self, pos: [f32; 2], cond: impl OptionOwned<Cond>, pivot: impl OptionOwned<[f32; 2]>) {
    unsafe {
      imgui::set_next_window_pos(pos, cond, pivot);
    }
  }
  #[assert_imgui_context]
  fn set_next_window_size(&mut self, size: [f32; 2], cond: impl OptionOwned<Cond>) {
    unsafe {
      imgui::set_next_window_size(size, cond);
    }
  }
  #[assert_imgui_context]
  fn get_window_pos(&mut self) -> (f32, f32) {
    unsafe {
      imgui::get_window_pos()
    }
  }

  #[assert_imgui_context]
  fn begin<'a, D: Display>(&mut self, title: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>) {
    unsafe {
      imgui::begin(title, open, flags);
    }
  }
  #[assert_imgui_context]
  fn end(&mut self) {
    unsafe {
      imgui::end();
    }
  }
  #[assert_imgui_context]
  fn begin_child<'a, D: Display>(&mut self, id: D, size: impl OptionOwned<[f32; 2]>, child_flags: impl OptionRef<'a, ChildFlags>, window_flags: impl OptionRef<'a, WindowFlags>) {
    unsafe {
      imgui::begin_child(id, size, child_flags, window_flags);
    }
  }
  #[assert_imgui_context]
  fn end_child(&mut self) {
    unsafe {
      imgui::end_child();
    }
  }
  #[assert_imgui_context]
  fn open_popup<'a, D: Display>(&mut self, id: D, flags: impl OptionRef<'a, PopupFlags>) {
    unsafe {
      imgui::open_popup(id, flags);
    }
  }
  #[assert_imgui_context]
  fn begin_popup<'a, D: Display>(&mut self, id: D, flags: impl OptionRef<'a, WindowFlags>) -> bool {
    unsafe {
      imgui::begin_popup(id, flags)
    }
  }
  #[assert_imgui_context]
  fn begin_popup_modal<'a, D: Display>(&mut self, id: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>) -> bool {
    unsafe {
      imgui::begin_popup_modal(id, open, flags)
    }
  }
  #[assert_imgui_context]
  fn end_popup(&mut self) {
    unsafe {
      imgui::end_popup();
    }
  }

  #[assert_imgui_context]
  fn push_id(&mut self, id: i32) {
    unsafe {
      imgui::push_id(id);
    }
  }
  #[assert_imgui_context]
  fn pop_id(&mut self) {
    unsafe {
      imgui::pop_id();
    }
  }

  #[assert_imgui_context]
  fn separator(&mut self) {
    unsafe {
      imgui::separator();
    }
  }
  #[assert_imgui_context]
  fn separator_text<D: Display>(&mut self, text: D) {
    unsafe {
      imgui::separator_text(text);
    }
  }

  #[assert_imgui_context]
  fn columns<D: Display>(&mut self, count: i32, id: impl OptionOwned<D>, border: impl OptionOwned<bool>) {
    unsafe {
      imgui::columns(count, id, border);
    }
  }
  #[assert_imgui_context]
  fn table_setup_column<'a, D: Display>(&mut self, label: D, flags: impl OptionRef<'a, TableColumnFlags>, width_or_weight: impl OptionOwned<f32>, user_id: impl OptionOwned<u32>) {
    unsafe {
      imgui::table_setup_column(label, flags, width_or_weight, user_id);
    }
  }
  #[assert_imgui_context]
  fn table_headers_row(&mut self) {
    unsafe {
      imgui::table_headers_row();
    }
  }
  #[assert_imgui_context]
  fn begin_table<'a, D: Display>(&mut self, id: D, columns: i32, flags: impl OptionRef<'a, TableFlags>) -> bool {
    unsafe {
      imgui::begin_table(id, columns, flags)
    }
  }
  #[assert_imgui_context]
  fn table_set_column_index(&mut self, column: i32) -> bool {
    unsafe {
      imgui::table_set_column_index(column)
    }
  }
  #[assert_imgui_context]
  fn table_next_row<'a>(&mut self, flags: impl OptionRef<'a, TableRowFlags>, min_row_height: impl OptionOwned<f32>) {
    unsafe {
      imgui::table_next_row(flags, min_row_height);
    }
  }
  #[assert_imgui_context]
  fn table_next_column(&mut self) -> bool {
    unsafe {
      imgui::table_next_column()
    }
  }
  #[assert_imgui_context]
  fn end_table(&mut self) {
    unsafe {
      imgui::end_table();
    }
  }

  #[assert_imgui_context]
  fn text<D: Display>(&mut self, text: D) {
    unsafe {
      imgui::text(text);
    }
  }
  #[assert_imgui_context]
  fn text_wrapped<D: Display>(&mut self, text: D) {
    unsafe {
      imgui::text_wrapped(text);
    }
  }
  #[assert_imgui_context]
  fn button<D: Display>(&mut self, label: D, size: impl OptionOwned<[f32; 2]>) -> bool {
    unsafe {
      imgui::button(label, size)
    }
  }
  #[assert_imgui_context]
  fn checkbox<D: Display>(&mut self, label: D, value: &mut bool) -> bool {
    unsafe {
      imgui::checkbox(label, value)
    }
  }
  #[assert_imgui_context]
  fn input_text<'a, D: Display>(&mut self, label: D, string: &mut String, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool {
    unsafe {
      imgui::input_text(label, string, flags, callback, user_data)  
    }
  }
  #[assert_imgui_context]
  fn input_text_multi_line<'a, D: Display>(&mut self, label: D, string: &mut String, size: impl OptionOwned<[f32; 2]>, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool {
    unsafe {
      imgui::input_text_multiline(label, string, size, flags, callback, user_data)
    }
  }

}




