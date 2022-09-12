/*
 * File: enum_ip_address_kind.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[derive(Debug)]
pub enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
