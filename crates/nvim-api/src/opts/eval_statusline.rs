use derive_builder::Builder;
use nvim_types::Object;

use crate::Window;

/// Options passed to [`api::eval_statusline`](crate::eval_statusline).
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct EvalStatuslineOpts {
    /// Character used to fill blank spaces in the statusline.
    #[builder(setter(strip_option))]
    fillchar: Option<char>,

    /// Return statuline informations from
    /// [`api::eval_statusline`](crate::eval_statusline).
    #[builder(setter(strip_option))]
    highlights: Option<bool>,

    /// Maximum width for the statusline.
    #[builder(setter(strip_option))]
    maxwidth: Option<u32>,

    /// Evaluate the tabline instead of the statusline. When `true` the
    /// [`window`](EvalStatuslineOptsBuilder::window) field is ignored.
    #[builder(setter(strip_option))]
    use_tabline: Option<bool>,

    /// Evaluate the winbar instead of the statusline. Mutually exclusive with
    /// [`use_tabline`](EvalStatuslineOptsBuilder::use_tabline).
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "neovim-0-8", feature = "neovim-nightly")))
    )]
    #[builder(setter(strip_option))]
    use_winbar: Option<bool>,

    /// Window to use as context for the statusline.
    #[builder(setter(into, strip_option))]
    window: Option<Window>,
}

impl EvalStatuslineOpts {
    #[inline(always)]
    /// Creates a new [`EvalStatuslineOptsBuilder`].
    pub fn builder() -> EvalStatuslineOptsBuilder {
        EvalStatuslineOptsBuilder::default()
    }
}

impl EvalStatuslineOptsBuilder {
    pub fn build(&mut self) -> EvalStatuslineOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Default)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_eval_statusline {
    winid: Object,
    fillchar: Object,
    maxwidth: Object,
    highlights: Object,
    #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
    use_winbar: Object,
    use_tabline: Object,
}

impl From<&EvalStatuslineOpts> for KeyDict_eval_statusline {
    fn from(opts: &EvalStatuslineOpts) -> Self {
        Self {
            winid: opts.window.as_ref().into(),
            fillchar: opts.fillchar.into(),
            maxwidth: opts.maxwidth.into(),
            highlights: opts.highlights.into(),
            #[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
            use_winbar: opts.use_winbar.into(),
            use_tabline: opts.use_tabline.into(),
        }
    }
}
