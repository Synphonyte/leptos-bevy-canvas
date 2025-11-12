# Leptos Bevy Canvas

[![Crates.io](https://img.shields.io/crates/v/leptos-bevy-canvas.svg)](https://crates.io/crates/leptos-bevy-canvas)
[![Docs](https://docs.rs/leptos-bevy-canvas/badge.svg)](https://docs.rs/leptos-bevy-canvas/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/leptos-bevy-canvas#license)
[![Build Status](https://github.com/synphonyte/leptos-bevy-canvas/actions/workflows/ci.yml/badge.svg)](https://github.com/synphonyte/leptos-bevy-canvas/actions/workflows/ci.yml)

<!-- cargo-rdme start -->

Embed an idiomatic Bevy app inside your Leptos app.

[Send and Receive Messages ![Messages Demo](https://media.githubusercontent.com/media/Synphonyte/leptos-bevy-canvas/refs/heads/main/docs/unidir_messages.webp)](https://github.com/Synphonyte/leptos-bevy-canvas/tree/main/examples/unidir_messages)

[Sync Bevy Queries ![Query Sync Demo](https://media.githubusercontent.com/media/Synphonyte/leptos-bevy-canvas/refs/heads/main/docs/synced_bevy_query.webp)](https://github.com/Synphonyte/leptos-bevy-canvas/tree/main/examples/synced_bevy_query)

<!-- [Sync Bevy States ![State Sync Loading Demo](https://media.githubusercontent.com/media/Synphonyte/leptos-bevy-canvas/refs/heads/main/docs/loading_screen.webp)](https://github.com/Synphonyte/leptos-bevy-canvas/tree/main/examples/loading_screen) -->

## Features

- **Easy to use** - Simply embed your Bevy app inside your Leptos app with the
  [`BevyCanvas`](fn@crate::prelude::BevyCanvas) component.
- **Idiomatic** - This crate doesn't want you to do anything differently in the way you write
  your Bevy app or your Leptos app. It just gives you the tools for them to communicate.
- **Messages** - Send messages in either or both directions between your Bevy app and your Leptos app.
- **Resource signals** - Synchronize Bevy `Resource`s with `RwSignal`s in your Leptos app.
- **Query signals** - Synchronize Bevy `Query`s with `RwSignal`s in your Leptos app.
- **State signals** - Synchronize Bevy `State`s with `RwSignal`s in your Leptos app.

## Example

```rust
use bevy::prelude::*;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[derive(Message)]
pub struct TextMessage {
    pub text: String,
}

#[component]
pub fn App() -> impl IntoView {
    // This initializes a sender for the Leptos app and
    // a receiver for the Bevy app
    let (text_message_sender, bevy_text_receiver) = message_l2b::<TextMessage>();

    let on_input = move |evt| {
        // send the message over to Bevy
        text_message_sender
            .send(TextMessage { text: event_target_value(&evt) })
            .ok();
    };

    view! {
        <input type="text" on:input=on_input />

        <BevyCanvas
            init=move || {
                // Pass the receiver into the Bevy app initialization
                init_bevy_app(bevy_text_receiver)
            }

            {..}
            width="300"
            height="500"
        />
    }
}

// In Bevy it ends up just as a normal message
pub fn set_text(
    mut message_reader: MessageReader<TextMessage>,
) {
    for message in message_reader.read() {
        // do something with the message
    }
}

// This initializes a normal Bevy app
fn init_bevy_app( text_receiver: BevyMessageReceiver<TextMessage>) -> App {
    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // "#bevy_canvas" is the default and can be
                    // changed in the <BevyCanvas> component
                    canvas: Some("#bevy_canvas".into()),
                    ..default()
                }),
                ..default()
            }),
        )
        // import the message here into Bevy
        .import_message_from_leptos(text_receiver)
        .add_systems(Update, set_text);

    app
}
```

Please check the examples to see how to synchronize a `Resource` or a `Query`.

## Compatibility

| Crate version | Compatible Leptos version | Compatible Bevy version |
|---------------|---------------------------|-------------------------|
| 0.4           | 0.8                       | 0.17                    |
| 0.3           | 0.8                       | 0.16                    |
| 0.1, 0.2      | 0.7                       | 0.15                    |

<!-- cargo-rdme end -->
