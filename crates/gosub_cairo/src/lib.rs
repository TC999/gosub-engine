#[cfg(feature = "cairo")]
use crate::elements::border::{GsBorder, GsBorderRadius, GsBorderSide};
#[cfg(feature = "cairo")]
use crate::elements::brush::GsBrush;
#[cfg(feature = "cairo")]
use crate::elements::color::GsColor;
#[cfg(feature = "cairo")]
use crate::elements::gradient::GsGradient;
#[cfg(feature = "cairo")]
use crate::elements::image::GsImage;
#[cfg(feature = "cairo")]
use crate::elements::rect::GsRect;
#[cfg(feature = "cairo")]
use crate::elements::text::GsText;
#[cfg(feature = "cairo")]
use crate::elements::transform::GsTransform;
#[cfg(feature = "cairo")]
use crate::render::window::{ActiveWindowData, WindowData};
use gosub_interface::render_backend::{RenderBackend, RenderRect, RenderText, Scene as _, WindowHandle};
use gosub_shared::geo::SizeU32;
use gosub_shared::types::Result;
pub use image::*;
#[cfg(feature = "cairo")]
use log::info;
#[cfg(feature = "cairo")]
pub use scene::*;
use std::fmt::Debug;

#[cfg(feature = "cairo")]
mod debug;
#[cfg(feature = "cairo")]
mod elements;
#[cfg(feature = "cairo")]
#[allow(unused)]
pub mod render;
#[cfg(feature = "cairo")]
mod scene;

#[derive(Clone)]
pub struct CairoBackend;

impl Debug for CairoBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CairoRenderer").finish()
    }
}

#[cfg(feature = "cairo")]
impl RenderBackend for CairoBackend {
    type Rect = GsRect;
    type Border = GsBorder;
    type BorderSide = GsBorderSide;
    type BorderRadius = GsBorderRadius;
    type Transform = GsTransform;
    type Gradient = GsGradient;
    type Color = GsColor;
    type Image = GsImage;
    type Brush = GsBrush;
    type Scene = Scene;
    type Text = GsText;
    type SVGRenderer = gosub_svg::resvg::Resvg;
    type FontManager = gosub_fontmanager::FontManager;

    type ActiveWindowData<'a> = ActiveWindowData;
    type WindowData<'a> = WindowData;

    fn draw_rect(&mut self, data: &mut Self::WindowData<'_>, rect: &RenderRect<Self>) {
        data.scene.draw_rect(rect);
    }

    fn draw_text(&mut self, data: &mut Self::WindowData<'_>, text: &RenderText<Self>) {
        data.scene.draw_text(text);
    }

    fn apply_scene(
        &mut self,
        data: &mut Self::WindowData<'_>,
        scene: &Self::Scene,
        transform: Option<Self::Transform>,
    ) {
        data.scene.apply_scene(scene, transform);
    }

    fn reset(&mut self, data: &mut Self::WindowData<'_>) {
        data.scene.reset();
    }

    fn activate_window<'a>(
        &mut self,
        _handle: impl WindowHandle + 'a,
        data: &mut Self::WindowData<'_>,
        _size: SizeU32,
    ) -> Result<Self::ActiveWindowData<'a>> {
        // I don't know what we need to do here. We have a handle (what does it hold?), the data (it
        // holds the current main scene), and a size. It seems that the active window data just contains
        // a cairo context. But we don't really need this in the active window data until we call
        // self::render().. Maybe we get the context here, and not in the scene::render()?
        Ok(ActiveWindowData {
            cr: data.cr.as_ref().unwrap().clone(),
        })
    }

    fn suspend_window(
        &mut self,
        _handle: impl WindowHandle,
        _data: &mut Self::ActiveWindowData<'_>,
        _window_data: &mut Self::WindowData<'_>,
    ) -> Result<()> {
        Ok(())
    }

    fn create_window_data<'a>(&mut self, _handle: impl WindowHandle) -> Result<Self::WindowData<'a>> {
        Ok(WindowData {
            scene: Scene::new(),
            cr: None,
        })
    }

    fn resize_window(
        &mut self,
        _window_data: &mut Self::WindowData<'_>,
        _active_window_data: &mut Self::ActiveWindowData<'_>,
        _size: SizeU32,
    ) -> Result<()> {
        info!("CairoBackend::resize_window()");
        Ok(())
    }

    fn render(
        &mut self,
        window_data: &mut Self::WindowData<'_>,
        active_data: &mut Self::ActiveWindowData<'_>,
    ) -> Result<()> {
        window_data.scene.render_to_context(&active_data.cr);
        Ok(())
    }
}

#[cfg(not(feature = "cairo"))]
impl RenderBackend for CairoBackend {
    type Rect = ();
    type Border = ();
    type BorderSide = ();
    type BorderRadius = ();
    type Transform = ();
    type Gradient = ();
    type Color = ();
    type Image = ();
    type Brush = ();
    type Scene = ();
    type Text = ();
    type SVGRenderer = ();
    type FontManager = ();

    type ActiveWindowData<'a> = ();
    type WindowData<'a> = ();

    fn draw_rect(&mut self, _data: &mut Self::WindowData<'_>, _rect: &RenderRect<Self>) {
        // No-op implementation for Windows
    }

    fn draw_text(&mut self, _data: &mut Self::WindowData<'_>, _text: &RenderText<Self>) {
        // No-op implementation for Windows
    }

    fn apply_scene(
        &mut self,
        _data: &mut Self::WindowData<'_>,
        _scene: &Self::Scene,
        _transform: Option<Self::Transform>,
    ) {
        // No-op implementation for Windows
    }

    fn reset(&mut self, _data: &mut Self::WindowData<'_>) {
        // No-op implementation for Windows
    }

    fn activate_window<'a>(
        &mut self,
        _handle: impl WindowHandle + 'a,
        _data: &mut Self::WindowData<'_>,
        _size: SizeU32,
    ) -> Result<Self::ActiveWindowData<'a>> {
        Ok(())
    }

    fn suspend_window(
        &mut self,
        _handle: impl WindowHandle,
        _data: &mut Self::ActiveWindowData<'_>,
        _window_data: &mut Self::WindowData<'_>,
    ) -> Result<()> {
        Ok(())
    }

    fn create_window_data<'a>(&mut self, _handle: impl WindowHandle) -> Result<Self::WindowData<'a>> {
        Ok(())
    }

    fn resize_window(
        &mut self,
        _window_data: &mut Self::WindowData<'_>,
        _active_window_data: &mut Self::ActiveWindowData<'_>,
        _size: SizeU32,
    ) -> Result<()> {
        Ok(())
    }

    fn render(
        &mut self,
        _window_data: &mut Self::WindowData<'_>,
        _active_data: &mut Self::ActiveWindowData<'_>,
    ) -> Result<()> {
        Ok(())
    }
}

impl CairoBackend {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for CairoBackend {
    fn default() -> Self {
        Self::new()
    }
}
