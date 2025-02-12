use azul_core::window::WindowCreateOptions;
use webrender::RendererOptions as WrRendererOptions;
use webrender::ProgramCache as WrProgramCache;
use webrender::ShaderPrecacheFlags as WrShaderPrecacheFlags;
use webrender::api::RenderNotifier as WrRenderNotifier;
use webrender::api::DocumentId as WrDocumentId;
use webrender::Shaders as WrShaders;
use std::rc::Rc;
use std::cell::RefCell;

pub(crate) mod process;
#[cfg(target_os = "windows")]
pub mod win32;
#[cfg(target_os = "linux")]
pub mod x11;
#[cfg(target_os = "macos")]
pub mod appkit;

// TODO: Cache compiled shaders between renderers
const WR_SHADER_CACHE: Option<&Rc<RefCell<WrShaders>>> = None;

fn default_renderer_options(options: &WindowCreateOptions) -> WrRendererOptions {
    use crate::wr_translate::wr_translate_debug_flags;
    WrRendererOptions {
        resource_override_path: None,
        use_optimized_shaders: true,
        enable_aa: true,
        enable_subpixel_aa: true,
        force_subpixel_aa: true,
        clear_color: webrender::api::ColorF {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }, // transparent
        panic_on_gl_error: false,
        precache_flags: WrShaderPrecacheFlags::EMPTY,
        cached_programs: Some(WrProgramCache::new(None)),
        enable_multithreading: true,
        debug_flags: wr_translate_debug_flags(&options.state.debug_state),
        ..WrRendererOptions::default()
    }
}

struct Notifier {}

impl WrRenderNotifier for Notifier {
    fn clone(&self) -> Box<dyn WrRenderNotifier> {
        Box::new(Notifier {})
    }
    fn wake_up(&self, composite_needed: bool) {}
    fn new_frame_ready(
        &self,
        _: WrDocumentId,
        _scrolled: bool,
        composite_needed: bool,
        _render_time: Option<u64>,
    ) {
    }
}

