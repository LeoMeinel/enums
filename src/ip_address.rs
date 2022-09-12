/*
 * File: ip_address.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub use self::enum_ip_address_kind::IpAddressKind;

mod enum_ip_address_kind;

pub(crate) fn ip_address() {
    let localhost_v4 = IpAddressKind::V4(127, 0, 0, 1);
    let localhost_v6 = IpAddressKind::V6(String::from("::1"));
    println!(
        "localhost in ipv4 is: {:#?}, localhost in ipv6 is: {:#?}",
        localhost_v4, localhost_v6
    );
}
