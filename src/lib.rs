
use std::{fmt::Display};

pub use imgui::{ self, * };


pub mod provider;


pub trait GuiProvider {

  fn push_style_color(&mut self, idx: ColorVar, color: impl Into<Color>);
  fn pop_style_color(&mut self, n: usize);
  fn push_style_var(&mut self, idx: StyleVar, x: f32, y: impl OptionOwned<f32>);
  fn pop_style_var(&mut self, n: usize);

  fn get_content_region_avail(&mut self) -> (f32, f32);
  fn is_item_clicked(&mut self, mouse_button: impl OptionOwned<MouseButton>) -> bool;

  fn set_cursor_pos(&mut self, x: f32, y: f32);
  fn set_next_item_width(&mut self, w: f32);
  fn same_line(&mut self, offset: impl OptionOwned<f32>, spacing: impl OptionOwned<f32>);
  fn push_item_width(&mut self, w: f32);
  fn set_next_window_pos(&mut self, pos: [f32; 2], cond: impl OptionOwned<Cond>, pivot: impl OptionOwned<[f32; 2]>);
  fn set_next_window_size(&mut self, size: [f32; 2], cond: impl OptionOwned<Cond>);
  fn get_window_pos(&mut self) -> (f32, f32);

  fn begin<'a, D: Display>(&mut self, title: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>);
  fn end(&mut self);
  fn begin_child<'a, D: Display>(&mut self, id: D, size: impl OptionOwned<[f32; 2]>, child_flags: impl OptionRef<'a, ChildFlags>, window_flags: impl OptionRef<'a, WindowFlags>);
  fn end_child(&mut self);
  fn open_popup<'a, D: Display>(&mut self, id: D, flags: impl OptionRef<'a, PopupFlags>);
  fn begin_popup<'a, D: Display>(&mut self, id: D, flags: impl OptionRef<'a, WindowFlags>) -> bool;
  fn begin_popup_modal<'a, D: Display>(&mut self, id: D, open: impl OptionMut<'a, bool>, flags: impl OptionRef<'a, WindowFlags>) -> bool;
  fn end_popup(&mut self);

  fn push_id(&mut self, id: i32);
  fn pop_id(&mut self);

  fn separator(&mut self);
  fn separator_text<D: Display>(&mut self, text: D);

  fn columns<D: Display + 'static>(&mut self, count: i32, id: impl OptionOwned<D>, border: impl OptionOwned<bool>);
  fn table_setup_column<'a, D: Display>(&mut self, label: D, flags: impl OptionRef<'a, TableColumnFlags>, width_or_weight: impl OptionOwned<f32>, user_id: impl OptionOwned<u32>);
  fn table_headers_row(&mut self);
  fn begin_table<'a, D: Display>(&mut self, id: D, columns: i32, flags: impl OptionRef<'a, TableFlags>) -> bool;
  fn table_set_column_index(&mut self, column: i32) -> bool;
  fn table_next_row<'a>(&mut self, flags: impl OptionRef<'a, TableRowFlags>, min_row_height: impl OptionOwned<f32>);
  fn table_next_column(&mut self) -> bool;
  fn end_table(&mut self);

  fn text<D: Display>(&mut self, text: D);
  fn text_wrapped<D: Display>(&mut self, text: D);
  fn button<D: Display>(&mut self, label: D, size: impl OptionOwned<[f32; 2]>) -> bool;
  fn checkbox<D: Display>(&mut self, label: D, value: &mut bool) -> bool;
  fn input_text<'a, D: Display>(&mut self, label: D, string: &mut String, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool;
  fn input_text_multi_line<'a, D: Display>(&mut self, label: D, string: &mut String, size: impl OptionOwned<[f32; 2]>, flags: impl OptionRef<'a, InputTextFlags>, callback: impl OptionOwned<fn()>, user_data: impl OptionOwned<*const ()>) -> bool;

}


//pub trait GuiGrid<V, T>: Into<Vec<V>> where V: Into<Vec<impl GuiOption<T>>> { }
//impl<V, T, U> GuiGrid<V, T> for U where U: Into<Vec<V>>, V: Into<Vec<impl GuiOption<T>>> { }

pub trait ExtendedGuiProvider: GuiProvider {

  fn color_button<D: Display>(&mut self, label: D, size: impl OptionOwned<[f32; 2]>, color: Color, light: Color, dark: Color) -> bool {
    self.push_style_color(ColorVar::Button, dark);
    self.push_style_color(ColorVar::ButtonHovered, light);
    self.push_style_color(ColorVar::ButtonActive, color);
    let res = self.button(label, size);
    self.pop_style_color(3);
    res
  }
  fn red_button<D: Display>(&mut self, label: D, size: impl OptionOwned<[f32; 2]>) -> bool {
    self.color_button(label, size, Color::RED, Color::LIGHT_RED, Color::DARK_RED)
  }
  fn gray_button<D: Display>(&mut self, label: D, size: impl OptionOwned<[f32; 2]>) -> bool {
    self.color_button(label, size, Color::GRAY, Color::LIGHT_GRAY, Color::DARK_GRAY)
  }

  fn grid<'a, D, const C: usize, const R: usize, F>(&mut self, id: D, rows: [[impl OptionOwned<F>; C]; R], flags: impl OptionRef<'a, TableFlags>) 
  where 
    D: Display,
    F: FnOnce(),
  {

    if self.begin_table(id, C as i32, flags) {
      for row in rows {
        for item in row {
          if let Some(_f) = item.into() {
            //f()
          }
          self.table_next_column();
        }
      }
      self.end_table();
    }
  }

}

impl<T: GuiProvider> ExtendedGuiProvider for T { }





