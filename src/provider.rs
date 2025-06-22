
use std::{f32::consts::PI, fmt::Display};

use imgui_derive::assert_imgui_context;
use crate::{GuiCallback, GuiColor, GuiColorVar, GuiDisplay, GuiFlags, GuiOption, GuiProvider, GuiStyleVar};


pub struct Provider {

}
impl Provider {
  pub fn new() -> Self {
    Provider {  }
  }
}



impl GuiProvider for Provider {

  #[assert_imgui_context]
  fn push_style_color(&mut self, idx: GuiColorVar, color: impl GuiColor) {
    unsafe {
      imgui::push_style_color(idx, color.into());
    }
  }
  #[assert_imgui_context]
  fn pop_style_color(&mut self, n: usize) {
    unsafe {
      imgui::pop_style_color(n as i32);
    }
  }
  #[assert_imgui_context]
  fn push_style_var(&mut self, idx: GuiStyleVar, x: f32, y: impl GuiOption<f32>) {
    unsafe {
      imgui::push_style_var(idx, x, y.into());
    } 
  }
  #[assert_imgui_context]
  fn pop_style_var(&mut self, n: usize) {
    unsafe {
      imgui::pop_style_var(Some(n as i32));
    }
  }

  #[assert_imgui_context]
  fn get_content_region_avail(&mut self) -> (f32, f32) {
    unsafe {
      imgui::get_content_region_avail()
    }
  }
  #[assert_imgui_context]
  fn is_item_clicked(&mut self, mouse_button: impl GuiOption<i32>) -> bool {
    unsafe {
      imgui::is_item_clicked(mouse_button.into())
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
  fn same_line(&mut self, offset: impl GuiOption<f32>, spacing: impl GuiOption<f32>) {
    unsafe {
      imgui::same_line(offset.into(), spacing.into());
    }
  }
  #[assert_imgui_context]
  fn push_item_width(&mut self, w: f32) {
    unsafe {
      imgui::push_item_width(w);
    }
  }

  #[assert_imgui_context]
  fn begin<'a, D: Display + 'static>(&mut self, title: &D, open: impl GuiOption<&'a mut bool>, flags: impl GuiFlags) {
    unsafe {
      imgui::begin(title, open.into(), flags.gui_flags());
    }
  }
  #[assert_imgui_context]
  fn end(&mut self) {
    unsafe {
      imgui::end();
    }
  }
  #[assert_imgui_context]
  fn begin_child<'a, D: Display + 'static>(&mut self, id: &D, size: impl GuiOption<[f32; 2]>, child_flags: impl GuiFlags, window_flags: impl GuiFlags) {
    unsafe {
      imgui::begin_child(id, size.into(), child_flags.gui_flags(), window_flags.gui_flags());
    }
  }
  #[assert_imgui_context]
  fn end_child(&mut self) {
    unsafe {
      imgui::end_child();
    }
  }
  #[assert_imgui_context]
  fn open_popup<D: GuiDisplay>(&mut self, id: &D, flags: impl GuiFlags) {
    unsafe {
      imgui::open_popup(id, flags.gui_flags());
    }
  }
  #[assert_imgui_context]
  fn begin_popup<D: GuiDisplay>(&mut self, id: &D, flags: impl GuiFlags) -> bool {
    unsafe {
      imgui::begin_popup(id, flags.gui_flags())
    }
  }
  #[assert_imgui_context]
  fn begin_popup_modal<'a, D: GuiDisplay>(&mut self, id: &D, open: impl GuiOption<&'a mut bool>, flags: impl GuiFlags) -> bool {
    unsafe {
      imgui::begin_popup_modal(id, open.into(), flags.gui_flags())
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
  fn separator_text<D: GuiDisplay>(&mut self, text: &D) {
    unsafe {
      imgui::separator_text(text);
    }
  }

  #[assert_imgui_context]
  fn columns<'a, D: GuiDisplay>(&mut self, count: i32, id: impl GuiOption<&'a D>, border: impl GuiOption<bool>) {
    unsafe {
      imgui::columns(count, id.into(), border.into());
    }
  }
  #[assert_imgui_context]
  fn table_setup_column<D: GuiDisplay>(&mut self, label: &D, flags: impl GuiFlags, width_or_weight: impl GuiOption<f32>, user_id: impl GuiOption<u32>) {
    unsafe {
      imgui::table_setup_column(label, flags.gui_flags(), width_or_weight.into(), user_id.into());
    }
  }
  #[assert_imgui_context]
  fn table_headers_row(&mut self) {
    unsafe {
      imgui::table_headers_row();
    }
  }
  #[assert_imgui_context]
  fn begin_table<D: GuiDisplay>(&mut self, id: &D, columns: i32, flags: impl GuiFlags) -> bool {
    unsafe {
      imgui::begin_table(id, columns, flags.gui_flags())
    }
  }
  #[assert_imgui_context]
  fn table_set_column_index(&mut self, column: i32) -> bool {
    unsafe {
      imgui::table_set_column_index(column)
    }
  }
  #[assert_imgui_context]
  fn table_next_row(&mut self, flags: impl GuiFlags, min_row_height: impl GuiOption<f32>) {
    unsafe {
      imgui::table_next_row(flags.gui_flags(), min_row_height.into());
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
  fn text<D: GuiDisplay>(&mut self, text: &D) {
    unsafe {
      imgui::text(text);
    }
  }
  #[assert_imgui_context]
  fn text_wrapped<D: GuiDisplay>(&mut self, text: &D) {
    unsafe {
      imgui::text_wrapped(text);
    }
  }
  #[assert_imgui_context]
  fn button<'a, D: Display + 'static>(&mut self, label: &D, size: impl GuiOption<[f32; 2]>) -> bool {
    unsafe {
      imgui::button(label, size.into())
    }
  }
  #[assert_imgui_context]
  fn checkbox<D: GuiDisplay>(&mut self, label: &D, value: &mut bool) -> bool {
    unsafe {
      imgui::checkbox(label, value)
    }
  }
  #[assert_imgui_context]
  fn input_text<'a, D: GuiDisplay>(&mut self, label: &D, string: &mut String, flags: impl GuiFlags, callback: impl GuiOption<fn()>, user_data: impl GuiOption<*const ()>) -> bool {
    unsafe {
      imgui::input_text(label, string, flags.gui_flags(), callback.into(), user_data.into())  
    }
  }
  #[assert_imgui_context]
  fn input_text_multi_line<'a, D: GuiDisplay>(&mut self, label: &D, string: &mut String, size: impl GuiOption<[f32; 2]>, flags: impl GuiFlags, callback: impl GuiOption<fn()>, user_data: impl GuiOption<*const ()>) -> bool {
    unsafe {
      imgui::input_text_multi_line(label, string, size.into(), flags.gui_flags(), callback.into(), user_data.into())
    }
  }

}

/*
fn foo(p: &Provider) {
  p.window("title", None, None, |provider| {
    provider.text("hello");
  });
}

unsafe fn bar() {
  imgui::begin("title", None, None);
  imgui::text("hello");
  imgui::end();
}

fn baz(p: &Provider) {
  
  let win = Window::new();
  win.text("hello");

  
  p.window2(win);

}
  */



