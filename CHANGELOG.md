# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-03-19

### Breaking Changes 🛠

- Renamed `RwSignalResource` to `RwSignalSynced`
- Renamed `signal_resource` to `signal_synced`

### New Features 🎉

- `RwSignalSynced` now implements `Write` and `Copy` + `Clone`
- Add synced single queries via `single_query_signal()`


## [0.1.0] - 2024-12-03

- Updated to Leptos 0.7.0 and Bevy 0.15.0
- Added example and readme gif

## [0.1.0-alpha1] - 2024-09-12

- Added Bevy <-> Leptos events
- Added Resource <-> RwSignal syncing
- Added `BevyCanvas` Leptos component

## [0.3.0] - 2025-05-19

- Updated to Leptos 0.8.2 and Bevy 0.16.0
