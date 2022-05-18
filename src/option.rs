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
pub use self::enum_option::Option;

mod enum_option;

pub(crate) fn option() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("5");
    let none: Option<i32> = Option::None;
    println!(
        "some_number is: {:#?}, some_string is: {:#?}, none is: {:#?}",
        some_number, some_string, none
    );
}
