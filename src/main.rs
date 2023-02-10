// pub mod syntax_json;
pub mod syntax_rust;
// pub mod syntax_xml;

use lady_deirdre::lexis::{SourceCode, TokenBuffer, ToSpan};
use lady_deirdre::syntax::Node;
use lady_deirdre::syntax::SyntaxTree;

fn main() {
    let code = TokenBuffer::<syntax_rust::lexis::RustToken>::from(r###"
    use std::fmt;
use std::future::Future;

use futures::pin_mut;
use futures::stream::{Stream, StreamExt};
use tracing::Instrument;

use crate::html::{BaseComponent, Scope};
use crate::platform::fmt::BufStream;
use crate::platform::{LocalHandle, Runtime};

/// A Yew Server-side Renderer that renders on the current thread.
///
/// # Note
///
/// This renderer does not spawn its own runtime and can only be used when:
///
/// - `wasm-bindgen-futures` is selected as the backend of Yew runtime.
/// - running within a [`Runtime`](crate::platform::Runtime).
/// - running within a tokio [`LocalSet`](struct@tokio::task::LocalSet).
#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    props: COMP::Properties,
    hydratable: bool,
}

impl<COMP> Default for LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    fn default() -> Self{}
}

impl<COMP> LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    /// Creates a [LocalServerRenderer] with default properties.
    pub fn new() -> Self;
}

impl<COMP> LocalServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a [LocalServerRenderer] with custom properties.
    pub fn with_props(props: COMP::Properties) -> Self;

    /// Sets whether an the rendered result is hydratable.
    ///
    /// Defaults to `true`.
    ///
    /// When this is sets to `true`, the rendered artifact will include additional information
    /// to assist with the hydration process.
    pub fn hydratable(mut self, val: bool) -> Self;

    /// Renders Yew Application.
    pub async fn render(self) -> String;

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String);

    /// Renders Yew Application into a string Stream
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        name = "render",
        skip(self),
        fields(hydratable = self.hydratable),
    )]
    pub fn render_stream(self) -> impl Stream<Item = String>;
}

/// A Yew Server-side Renderer.
///
/// This renderer spawns the rendering task to a Yew [`Runtime`]. and receives result when
/// the rendering process has finished.
///
/// See [`yew::platform`] for more information.
#[cfg(feature = "ssr")]
pub struct ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    create_props: Box<dyn Send + FnOnce() -> COMP::Properties>,
    hydratable: bool,
    rt: Option<Runtime>,
}

impl<COMP> fmt::Debug for ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<COMP> Default for ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    fn default() -> Self;
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
    COMP::Properties: Default,
{
    /// Creates a [ServerRenderer] with default properties.
    pub fn new() -> Self;
}

impl<COMP> ServerRenderer<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a [ServerRenderer] with custom properties.
    ///
    /// # Note
    ///
    /// The properties does not have to implement `Send`.
    /// However, the function to create properties needs to be `Send`.
    pub fn with_props<F>(create_props: F) -> Self
    where
        F: 'static + Send + FnOnce() -> COMP::Properties;

    /// Sets the runtime the ServerRenderer will run the rendering task with.
    pub fn with_runtime(mut self, rt: Runtime) -> Self;

    /// Sets whether an the rendered result is hydratable.
    ///
    /// Defaults to `true`.
    ///
    /// When this is sets to `true`, the rendered artifact will include additional information
    /// to assist with the hydration process.
    pub fn hydratable(mut self, val: bool) -> Self;

    /// Renders Yew Application.
    pub async fn render(self) -> String;

    /// Renders Yew Application to a String.
    pub async fn render_to_string(self, w: &mut String);

    #[inline]
    fn spawn_rendering_task<F, Fut>(rt: Option<Runtime>, create_task: F)
    where
        F: 'static + Send + FnOnce() -> Fut,
        Fut: Future<Output = ()> + 'static;

    /// Renders Yew Application into a string Stream.
    pub fn render_stream(self) -> impl Send + Stream<Item = String>;
}"###);
    let tree = syntax_rust::syntax::RustNode::parse(code.cursor(..));
    println!("{}", tree.errors()
            .map(|error| format!("{}: {}", error.span().format(&code), error))
            .collect::<Vec<_>>()
            .join("\n"))

}