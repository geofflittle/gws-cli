// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Library exposure for embedders.
//!
//! The `gws` binary is the primary product of this crate. This `lib.rs`
//! re-exposes the CLI's internal modules so external Rust programs can
//! drive `auth::get_token` (and adjacent CLI helpers) without shelling
//! out to the binary. Pair with the `google-workspace` crate (HTTP +
//! discovery) for authenticated request issuance.
//!
//! Module visibility mirrors `main.rs` so the lib + bin compilation
//! roots stay in sync. The CLI's internal modules form a tightly-
//! coupled graph (auth → credential_store → output, plus auth_commands
//! shared utilities); exposing the full surface avoids API churn from
//! refactor-driven re-pluming.

pub mod auth;
pub mod auth_commands;
pub mod client;
pub mod commands;
pub mod credential_store;
pub mod discovery;
pub mod error;
pub mod executor;
pub mod formatter;
pub mod fs_util;
pub mod generate_skills;
pub mod helpers;
pub mod logging;
pub mod oauth_config;
pub mod output;
pub mod schema;
pub mod services;
pub mod setup;
pub mod setup_tui;
pub mod text;
pub mod timezone;
pub mod token_storage;
pub mod validate;
