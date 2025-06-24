
use std::{collections::HashSet, fmt::Display, hash::Hash, ops::Index};

use crate::provider::Provider;
pub type GuiColorVar = imgui::ImGuiCol;
pub type GuiStyleVar = imgui::ImGuiStyleVar;
pub use imgui;


pub mod provider;
//pub mod window;


pub trait GuiDisplay: Display + 'static { }
impl<T: Display + 'static> GuiDisplay for T { }
impl GuiDisplay for str { }

pub type Flags = Option<i32>;
pub trait GuiFlags: 'static {
  fn gui_flags(&self) -> Flags;
}
impl GuiFlags for i32 {
  fn gui_flags(&self) -> Flags {
    Some(*self)
  }
}
impl GuiFlags for isize {
  fn gui_flags(&self) -> Flags {
    (*self as i32).gui_flags()
  }
}
impl GuiFlags for () {
  fn gui_flags(&self) -> Flags {
    None
  }
}
impl GuiFlags for GuiChildFlags {
  fn gui_flags(&self) -> Flags {
    (*self as isize).gui_flags()
  }
}
impl GuiFlags for GuiWindowFlags {
  fn gui_flags(&self) -> Flags {
    (*self as isize).gui_flags()
  }
}
impl GuiFlags for GuiInputTextFlags {
  fn gui_flags(&self) -> Flags {
    (*self as isize).gui_flags()
  }
}
impl GuiFlags for GuiTableColumnFlags {
  fn gui_flags(&self) -> Flags {
    (*self as isize).gui_flags()
  }
}
impl GuiFlags for GuiTableFlags {
  fn gui_flags(&self) -> Flags {
    (*self as isize).gui_flags()
  }
}
impl<T: GuiFlags> GuiFlags for Option<T> {
  fn gui_flags(&self) -> Flags {
    let Some(flags) = self else {
      return None;
    };
    flags.gui_flags()
  }
}
impl<T: GuiFlags + Eq + Hash + Copy, const N: usize> GuiFlags for [T; N] {
  fn gui_flags(&self) -> Flags {
    HashSet::from(*self).gui_flags()
  }
}
impl<T: GuiFlags> GuiFlags for HashSet<T> {
  fn gui_flags(&self) -> Flags {
    self.iter()
      .filter_map(|e| e.gui_flags())
      .fold(0, |acc, el| acc | el)
      .into()
  }
}

pub trait GuiCallback: FnMut(Provider) -> Provider + 'static { }
impl<F> GuiCallback for F where F: FnMut(Provider) -> Provider + 'static { }

pub type Color = imgui::Color;
pub trait GuiColor: Into<Color> + 'static { }
impl<T: Into<Color> + 'static> GuiColor for T { }

pub trait GuiOption<T>: Into<Option<T>> + 'static { }
impl<T, U> GuiOption<T> for U where U: Into<Option<T>> + 'static { }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiChildFlags {
  None = imgui::ImGuiChildFlags_None as isize,
  Border = imgui::ImGuiChildFlags_Border as isize,
  AlwaysUseWindowPadding = imgui::ImGuiChildFlags_AlwaysUseWindowPadding as isize,
  ResizeX = imgui::ImGuiChildFlags_ResizeX as isize,
  ResizeY = imgui::ImGuiChildFlags_ResizeY as isize,
  AutoResizeX = imgui::ImGuiChildFlags_AutoResizeX as isize,
  AutoResizeY = imgui::ImGuiChildFlags_AutoResizeY as isize,
  AlwaysAutoResize = imgui::ImGuiChildFlags_AlwaysAutoResize as isize,
  FrameStyle = imgui::ImGuiChildFlags_FrameStyle as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiWindowFlags {
  None = imgui::ImGuiWindowFlags_None as isize,
  NoTitleBar = imgui::ImGuiWindowFlags_NoTitleBar as isize,
  NoResize = imgui::ImGuiWindowFlags_NoResize as isize,
  NoMove = imgui::ImGuiWindowFlags_NoMove as isize,
  NoScrollbar = imgui::ImGuiWindowFlags_NoScrollbar as isize,
  NoScrollWithMouse = imgui::ImGuiWindowFlags_NoScrollWithMouse as isize,
  NoCollapse = imgui::ImGuiWindowFlags_NoCollapse as isize,
  AlwaysAutoResize = imgui::ImGuiWindowFlags_AlwaysAutoResize as isize,
  NoBackground = imgui::ImGuiWindowFlags_NoBackground as isize,
  NoSavedSettings = imgui::ImGuiWindowFlags_NoSavedSettings as isize,
  NoMouseInputs = imgui::ImGuiWindowFlags_NoMouseInputs as isize,
  MenuBar = imgui::ImGuiWindowFlags_MenuBar as isize,
  HorizontalScrollbar = imgui::ImGuiWindowFlags_HorizontalScrollbar as isize,
  NoFocusOnAppearing = imgui::ImGuiWindowFlags_NoFocusOnAppearing as isize,
  NoBringToFrontOnFocus = imgui::ImGuiWindowFlags_NoBringToFrontOnFocus as isize,
  AlwaysVerticalScrollbar = imgui::ImGuiWindowFlags_AlwaysVerticalScrollbar as isize,
  AlwaysHorizontalScrollbar = imgui::ImGuiWindowFlags_AlwaysHorizontalScrollbar as isize,
  NoNavInputs = imgui::ImGuiWindowFlags_NoNavInputs as isize,
  NoNavFocus = imgui::ImGuiWindowFlags_NoNavFocus as isize,
  UnsavedDocument = imgui::ImGuiWindowFlags_UnsavedDocument as isize,
  NoDocking = imgui::ImGuiWindowFlags_NoDocking as isize,
  NoNav = imgui::ImGuiWindowFlags_NoNav as isize,
  NoDecoration = imgui::ImGuiWindowFlags_NoDecoration as isize,
  NoInputs = imgui::ImGuiWindowFlags_NoInputs as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiInputTextFlags {
  None = imgui::ImGuiInputTextFlags_None as isize,
  CharsDecimal = imgui::ImGuiInputTextFlags_CharsDecimal as isize,
  CharsHexadecimal = imgui::ImGuiInputTextFlags_CharsHexadecimal as isize,
  CharsUppercase = imgui::ImGuiInputTextFlags_CharsUppercase as isize,
  CharsNoBlank = imgui::ImGuiInputTextFlags_CharsNoBlank as isize,
  AutoSelectAll = imgui::ImGuiInputTextFlags_AutoSelectAll as isize,
  EnterReturnsTrue = imgui::ImGuiInputTextFlags_EnterReturnsTrue as isize,
  CallbackCompletion = imgui::ImGuiInputTextFlags_CallbackCompletion as isize,
  CallbackHistory = imgui::ImGuiInputTextFlags_CallbackHistory as isize,
  CallbackAlways = imgui::ImGuiInputTextFlags_CallbackAlways as isize,
  CallbackCharFilter = imgui::ImGuiInputTextFlags_CallbackCharFilter as isize,
  AllowTabInput = imgui::ImGuiInputTextFlags_AllowTabInput as isize,
  CtrlEnterForNewLine = imgui::ImGuiInputTextFlags_CtrlEnterForNewLine as isize,
  NoHorizontalScroll = imgui::ImGuiInputTextFlags_NoHorizontalScroll as isize,
  AlwaysOverwrite = imgui::ImGuiInputTextFlags_AlwaysOverwrite as isize,
  ReadOnly = imgui::ImGuiInputTextFlags_ReadOnly as isize,
  Password = imgui::ImGuiInputTextFlags_Password as isize,
  NoUndoRedo = imgui::ImGuiInputTextFlags_NoUndoRedo as isize,
  CharsScientific = imgui::ImGuiInputTextFlags_CharsScientific as isize,
  CallbackResize = imgui::ImGuiInputTextFlags_CallbackResize as isize,
  CallbackEdit = imgui::ImGuiInputTextFlags_CallbackEdit as isize,
  EscapeClearsAll = imgui::ImGuiInputTextFlags_EscapeClearsAll as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiTableColumnFlags {
  None = imgui::ImGuiTableColumnFlags_None as isize,
  Disabled = imgui::ImGuiTableColumnFlags_Disabled as isize,
  DefaultHide = imgui::ImGuiTableColumnFlags_DefaultHide as isize,
  DefaultSort = imgui::ImGuiTableColumnFlags_DefaultSort as isize,
  WidthStretch = imgui::ImGuiTableColumnFlags_WidthStretch as isize,
  WidthFixed = imgui::ImGuiTableColumnFlags_WidthFixed as isize,
  NoResize = imgui::ImGuiTableColumnFlags_NoResize as isize,
  NoReorder = imgui::ImGuiTableColumnFlags_NoReorder as isize,
  NoHide = imgui::ImGuiTableColumnFlags_NoHide as isize,
  NoClip = imgui::ImGuiTableColumnFlags_NoClip as isize,
  NoSort = imgui::ImGuiTableColumnFlags_NoSort as isize,
  NoSortAscending = imgui::ImGuiTableColumnFlags_NoSortAscending as isize,
  NoSortDescending = imgui::ImGuiTableColumnFlags_NoSortDescending as isize,
  NoHeaderLabel = imgui::ImGuiTableColumnFlags_NoHeaderLabel as isize,
  NoHeaderWidth = imgui::ImGuiTableColumnFlags_NoHeaderWidth as isize,
  PreferSortAscending = imgui::ImGuiTableColumnFlags_PreferSortAscending as isize,
  PreferSortDescending = imgui::ImGuiTableColumnFlags_PreferSortDescending as isize,
  IndentEnable = imgui::ImGuiTableColumnFlags_IndentEnable as isize,
  IndentDisable = imgui::ImGuiTableColumnFlags_IndentDisable as isize,
  AngledHeader = imgui::ImGuiTableColumnFlags_AngledHeader as isize,
  IsEnabled = imgui::ImGuiTableColumnFlags_IsEnabled as isize,
  IsVisible = imgui::ImGuiTableColumnFlags_IsVisible as isize,
  IsSorted = imgui::ImGuiTableColumnFlags_IsSorted as isize,
  IsHovered = imgui::ImGuiTableColumnFlags_IsHovered as isize,
  WidthMask_ = imgui::ImGuiTableColumnFlags_WidthMask_ as isize,
  IndentMask_ = imgui::ImGuiTableColumnFlags_IndentMask_ as isize,
  StatusMask_ = imgui::ImGuiTableColumnFlags_StatusMask_ as isize,
  NoDirectResize_ = imgui::ImGuiTableColumnFlags_NoDirectResize_ as isize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuiTableFlags {
  None = imgui::ImGuiTableFlags_None as isize,
  Resizable = imgui::ImGuiTableFlags_Resizable as isize,
  Reorderable = imgui::ImGuiTableFlags_Reorderable as isize,
  Hideable = imgui::ImGuiTableFlags_Hideable as isize,
  Sortable = imgui::ImGuiTableFlags_Sortable as isize,
  NoSavedSettings = imgui::ImGuiTableFlags_NoSavedSettings as isize,
  ContextMenuInBody = imgui::ImGuiTableFlags_ContextMenuInBody as isize,
  RowBg = imgui::ImGuiTableFlags_RowBg as isize,
  BordersInnerH = imgui::ImGuiTableFlags_BordersInnerH as isize,
  BordersOuterH = imgui::ImGuiTableFlags_BordersOuterH as isize,
  BordersInnerV = imgui::ImGuiTableFlags_BordersInnerV as isize,
  BordersOuterV = imgui::ImGuiTableFlags_BordersOuterV as isize,
  BordersH = imgui::ImGuiTableFlags_BordersH as isize,
  BordersV = imgui::ImGuiTableFlags_BordersV as isize,
  BordersInner = imgui::ImGuiTableFlags_BordersInner as isize,
  BordersOuter = imgui::ImGuiTableFlags_BordersOuter as isize,
  Borders = imgui::ImGuiTableFlags_Borders as isize,
  NoBordersInBody = imgui::ImGuiTableFlags_NoBordersInBody as isize,
  NoBordersInBodyUntilResize = imgui::ImGuiTableFlags_NoBordersInBodyUntilResize as isize,
  SizingFixedFit = imgui::ImGuiTableFlags_SizingFixedFit as isize,
  SizingFixedSame = imgui::ImGuiTableFlags_SizingFixedSame as isize,
  SizingStretchProp = imgui::ImGuiTableFlags_SizingStretchProp as isize,
  SizingStretchSame = imgui::ImGuiTableFlags_SizingStretchSame as isize,
  NoHostExtendX = imgui::ImGuiTableFlags_NoHostExtendX as isize,
  NoHostExtendY = imgui::ImGuiTableFlags_NoHostExtendY as isize,
  NoKeepColumnsVisible = imgui::ImGuiTableFlags_NoKeepColumnsVisible as isize,
  PreciseWidths = imgui::ImGuiTableFlags_PreciseWidths as isize,
  NoClip = imgui::ImGuiTableFlags_NoClip as isize,
  PadOuterX = imgui::ImGuiTableFlags_PadOuterX as isize,
  NoPadOuterX = imgui::ImGuiTableFlags_NoPadOuterX as isize,
  NoPadInnerX = imgui::ImGuiTableFlags_NoPadInnerX as isize,
  ScrollX = imgui::ImGuiTableFlags_ScrollX as isize,
  ScrollY = imgui::ImGuiTableFlags_ScrollY as isize,
  SortMulti = imgui::ImGuiTableFlags_SortMulti as isize,
  SortTristate = imgui::ImGuiTableFlags_SortTristate as isize,
  HighlightHoveredColumn = imgui::ImGuiTableFlags_HighlightHoveredColumn as isize,
  SizingMask_ = imgui::ImGuiTableFlags_SizingMask_ as isize,
  //gs_None = imgui::ImGuiTableRowFlags_None as isize,
  //gs_Headers = imgui::ImGuiTableRowFlags_Headers as isize,
}


pub trait GuiProvider {

  fn push_style_color(&mut self, idx: GuiColorVar, color: impl GuiColor);
  fn pop_style_color(&mut self, n: usize);
  fn push_style_var(&mut self, idx: GuiStyleVar, x: f32, y: impl GuiOption<f32>);
  fn pop_style_var(&mut self, n: usize);

  fn get_content_region_avail(&mut self) -> (f32, f32);
  fn is_item_clicked(&mut self, mouse_button: impl GuiOption<i32>) -> bool;

  fn set_cursor_pos(&mut self, x: f32, y: f32);
  fn set_next_item_width(&mut self, w: f32);
  fn same_line(&mut self, offset: impl GuiOption<f32>, spacing: impl GuiOption<f32>);
  fn push_item_width(&mut self, w: f32);

  fn begin<'a, D: GuiDisplay>(&mut self, title: &D, open: impl GuiOption<&'a mut bool>, flags: impl GuiFlags);
  fn end(&mut self);
  fn begin_child<D: GuiDisplay>(&mut self, id: &D, size: impl GuiOption<[f32; 2]>, child_flags: impl GuiFlags, window_flags: impl GuiFlags);
  fn end_child(&mut self);
  fn open_popup<D: GuiDisplay>(&mut self, id: &D, flags: impl GuiFlags);
  fn begin_popup<D: GuiDisplay>(&mut self, id: &D, flags: impl GuiFlags) -> bool;
  fn begin_popup_modal<'a, D: GuiDisplay>(&mut self, id: &D, open: impl GuiOption<&'a mut bool>, flags: impl GuiFlags) -> bool;
  fn end_popup(&mut self);

  fn push_id(&mut self, id: i32);
  fn pop_id(&mut self);

  fn separator(&mut self);
  fn separator_text<D: GuiDisplay>(&mut self, text: &D);

  fn columns<'a, D: GuiDisplay>(&mut self, count: i32, id: impl GuiOption<&'a D>, border: impl GuiOption<bool>);
  fn table_setup_column<D: GuiDisplay>(&mut self, label: &D, flags: impl GuiFlags, width_or_weight: impl GuiOption<f32>, user_id: impl GuiOption<u32>);
  fn table_headers_row(&mut self);
  fn begin_table<D: GuiDisplay>(&mut self, id: &D, columns: i32, flags: impl GuiFlags) -> bool;
  fn table_set_column_index(&mut self, column: i32) -> bool;
  fn table_next_row(&mut self, flags: impl GuiFlags, min_row_height: impl GuiOption<f32>);
  fn table_next_column(&mut self) -> bool;
  fn end_table(&mut self);

  fn text<D: GuiDisplay>(&mut self, text: &D);
  fn text_wrapped<D: GuiDisplay>(&mut self, text: &D);
  fn button<D: GuiDisplay>(&mut self, label: &D, size: impl GuiOption<[f32; 2]>) -> bool;
  fn checkbox<D: GuiDisplay>(&mut self, label: &D, value: &mut bool) -> bool;
  fn input_text<D: GuiDisplay>(&mut self, label: &D, string: &mut String, flags: impl GuiFlags, callback: impl GuiOption<fn()>, user_data: impl GuiOption<*const ()>) -> bool;
  fn input_text_multi_line<D: GuiDisplay>(&mut self, label: &D, string: &mut String, size: impl GuiOption<[f32; 2]>, flags: impl GuiFlags, callback: impl GuiOption<fn()>, user_data: impl GuiOption<*const ()>) -> bool;

}


//pub trait GuiGrid<V, T>: Into<Vec<V>> where V: Into<Vec<impl GuiOption<T>>> { }
//impl<V, T, U> GuiGrid<V, T> for U where U: Into<Vec<V>>, V: Into<Vec<impl GuiOption<T>>> { }

pub trait ExtendedGuiProvider: GuiProvider {

  fn color_button<D: GuiDisplay>(&mut self, label: &D, size: impl GuiOption<[f32; 2]>, color: Color, light: Color, dark: Color) -> bool {
    self.push_style_color(GuiColorVar::Button, dark);
    self.push_style_color(GuiColorVar::ButtonHovered, light);
    self.push_style_color(GuiColorVar::ButtonActive, color);
    let res = self.button(label, size);
    self.pop_style_color(3);
    res
  }
  fn red_button<D: GuiDisplay>(&mut self, label: &D, size: impl GuiOption<[f32; 2]>) -> bool {
    self.color_button(label, size, Color::Red, Color::LightRed, Color::DarkRed)
  }
  fn gray_button<D: GuiDisplay>(&mut self, label: &D, size: impl GuiOption<[f32; 2]>) -> bool {
    self.color_button(label, size, Color::Gray, Color::LightGray, Color::DarkGray)
  }

  fn grid<D, const C: usize, const R: usize, F>(&mut self, id: &D, rows: [[impl GuiOption<F>; C]; R], flags: impl GuiFlags) 
  where 
    D: GuiDisplay,
    F: FnOnce()
  {

    if self.begin_table(id, C as i32, flags.gui_flags()) {
      for row in rows {
        for item in row {
          if let Some(f) = item.into() {
            f()
          }
          self.table_next_column();
        }
      }
      self.end_table();
    }
  }

}

impl<T: GuiProvider> ExtendedGuiProvider for T { }





