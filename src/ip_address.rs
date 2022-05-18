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
