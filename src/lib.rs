//! Embed an idiomatic Bevy app inside your Leptos app.
//!
//! # Features
//!
//! - **Easy to use** - Simply embed your Bevy app inside your Leptos app with the
//!   [`BevyCanvas`](fn@crate::prelude::BevyCanvas) component.
//! - **Idiomatic** - This crate doesn't want you to do anything differently in the way you write
//!   your Bevy app or your Leptos app. It just gives you the tools for them to communicate.
//! - **Events** - Send events in either or both directions between your Bevy app and your Leptos app.
//! - **Resource signals** - Synchronize Bevy `Resource`s with `RwSignal`s in your Leptos app.
//!
//! # Example
//!
//! ```
//! use bevy::prelude::*;
//! use leptos::prelude::*;
//! use leptos_bevy_canvas::prelude::*;
//!
//! #[derive(Event)]
//! pub struct TextEvent {
//!     pub text: String,
//! }
//!
//! #[component]
//! pub fn App() -> impl IntoView {
//!     // This initializes a sender for the Leptos app and
//!     // a receiver for the Bevy app
//!     let (text_event_sender, bevy_text_receiver) = event_l2b::<TextEvent>();
//!
//!     let on_input = move |evt| {
//!         // send the event over to Bevy
//!         text_event_sender
//!             .send(TextEvent { text: event_target_value(&evt) })
//!             .ok();
//!     };
//!
//!     view! {
//!         <input type="text" on:input=on_input />
//!
//!         <BevyCanvas
//!             init=move || {
//!                 // Pass the receiver into the Bevy app initialization
//!                 init_bevy_app(bevy_text_receiver)
//!             }
//!
//!             {..}
//!             width="300"
//!             height="500"
//!         />
//!     }
//! }
//!
//! // In Bevy it ends up just as a normal event
//! pub fn set_text(
//!     mut event_reader: EventReader<TextEvent>,
//! ) {
//!     for event in event_reader.read() {
//!         // do something with the event
//!     }
//! }
//!
//! // This initializes a normal Bevy app
//! fn init_bevy_app( text_receiver: BevyEventReceiver<TextEvent>) -> App {
//!     let mut app = App::new();
//!     app
//!         .add_plugins(
//!             DefaultPlugins.set(WindowPlugin {
//!                 primary_window: Some(Window {
//!                     // "#bevy_canvas" is the default and can be
//!                     // changed in the <BevyCanvas> component
//!                     canvas: Some("#bevy_canvas".into()),
//!                     ..default()
//!                 }),
//!                 ..default()
//!             }),
//!         )
//!         // import the event here into Bevy
//!         .import_event_from_leptos(text_receiver)
//!         .add_systems(Update, set_text);
//!
//!     app
//! }
//! ```

mod app_extension;
mod events;
mod leptos_component;
mod resources;
pub mod systems;
pub mod traits;
mod utils;

pub mod prelude {
    pub use crate::app_extension::*;
    pub use crate::events::*;
    pub use crate::leptos_component::*;
    pub use crate::resources::*;
}
