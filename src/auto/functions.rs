// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use pango;


pub fn context_get_resolution<P: IsA<pango::Context>>(context: &P) -> f64 {
    unsafe {
        ffi::pango_cairo_context_get_resolution(context.as_ref().to_glib_none().0)
    }
}

//pub fn context_get_shape_renderer<P: IsA<pango::Context>>(context: &P, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Fn(&cairo::Context, /*Ignored*/pango::AttrShape, bool, /*Unimplemented*/Fundamental: Pointer) {
//    unsafe { TODO: call ffi::pango_cairo_context_get_shape_renderer() }
//}

pub fn context_set_font_options<'a, P: IsA<pango::Context>, Q: Into<Option<&'a cairo::FontOptions>>>(context: &P, options: Q) {
    let options = options.into();
    unsafe {
        ffi::pango_cairo_context_set_font_options(context.as_ref().to_glib_none().0, options.to_glib_none().0);
    }
}

pub fn context_set_resolution<P: IsA<pango::Context>>(context: &P, dpi: f64) {
    unsafe {
        ffi::pango_cairo_context_set_resolution(context.as_ref().to_glib_none().0, dpi);
    }
}

//pub fn context_set_shape_renderer<P: IsA<pango::Context>>(context: &P, func: /*Unimplemented*/Fn(&cairo::Context, /*Ignored*/pango::AttrShape, bool, /*Unimplemented*/Fundamental: Pointer), data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call ffi::pango_cairo_context_set_shape_renderer() }
//}

pub fn create_context(cr: &cairo::Context) -> Option<pango::Context> {
    unsafe {
        from_glib_full(ffi::pango_cairo_create_context(mut_override(cr.to_glib_none().0)))
    }
}

pub fn create_layout(cr: &cairo::Context) -> Option<pango::Layout> {
    unsafe {
        from_glib_full(ffi::pango_cairo_create_layout(mut_override(cr.to_glib_none().0)))
    }
}

pub fn error_underline_path(cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        ffi::pango_cairo_error_underline_path(mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn glyph_string_path<P: IsA<pango::Font>>(cr: &cairo::Context, font: &P, glyphs: &mut pango::GlyphString) {
    unsafe {
        ffi::pango_cairo_glyph_string_path(mut_override(cr.to_glib_none().0), font.as_ref().to_glib_none().0, glyphs.to_glib_none_mut().0);
    }
}

pub fn layout_line_path(cr: &cairo::Context, line: &pango::LayoutLine) {
    unsafe {
        ffi::pango_cairo_layout_line_path(mut_override(cr.to_glib_none().0), line.to_glib_none().0);
    }
}

pub fn layout_path<P: IsA<pango::Layout>>(cr: &cairo::Context, layout: &P) {
    unsafe {
        ffi::pango_cairo_layout_path(mut_override(cr.to_glib_none().0), layout.as_ref().to_glib_none().0);
    }
}

pub fn show_error_underline(cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        ffi::pango_cairo_show_error_underline(mut_override(cr.to_glib_none().0), x, y, width, height);
    }
}

pub fn show_glyph_item(cr: &cairo::Context, text: &str, glyph_item: &mut pango::GlyphItem) {
    unsafe {
        ffi::pango_cairo_show_glyph_item(mut_override(cr.to_glib_none().0), text.to_glib_none().0, glyph_item.to_glib_none_mut().0);
    }
}

pub fn show_glyph_string<P: IsA<pango::Font>>(cr: &cairo::Context, font: &P, glyphs: &mut pango::GlyphString) {
    unsafe {
        ffi::pango_cairo_show_glyph_string(mut_override(cr.to_glib_none().0), font.as_ref().to_glib_none().0, glyphs.to_glib_none_mut().0);
    }
}

pub fn show_layout<P: IsA<pango::Layout>>(cr: &cairo::Context, layout: &P) {
    unsafe {
        ffi::pango_cairo_show_layout(mut_override(cr.to_glib_none().0), layout.as_ref().to_glib_none().0);
    }
}

pub fn show_layout_line(cr: &cairo::Context, line: &pango::LayoutLine) {
    unsafe {
        ffi::pango_cairo_show_layout_line(mut_override(cr.to_glib_none().0), line.to_glib_none().0);
    }
}

pub fn update_context<P: IsA<pango::Context>>(cr: &cairo::Context, context: &P) {
    unsafe {
        ffi::pango_cairo_update_context(mut_override(cr.to_glib_none().0), context.as_ref().to_glib_none().0);
    }
}

pub fn update_layout<P: IsA<pango::Layout>>(cr: &cairo::Context, layout: &P) {
    unsafe {
        ffi::pango_cairo_update_layout(mut_override(cr.to_glib_none().0), layout.as_ref().to_glib_none().0);
    }
}
