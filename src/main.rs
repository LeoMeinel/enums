/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::ip_address::ip_address;
use crate::message::message;
use crate::option::option;
use crate::option_default::default_option;
use crate::option_if_let_syntax::if_let_syntax;
use crate::option_pattern_matching::option_pattern_matching;
use crate::pattern_matching::pattern_matching;

mod ip_address;
mod message;
mod option;
mod option_default;
mod option_if_let_syntax;
mod option_pattern_matching;
mod pattern_matching;

fn main() {
    ip_address();
    message();
    option();
    default_option();
    pattern_matching();
    option_pattern_matching();
    if_let_syntax();
}
