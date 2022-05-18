/*
 * enums is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/enums/blob/main/LICENSE
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
