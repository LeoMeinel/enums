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

pub use self::enum_message::Message;

mod enum_message;

pub(crate) fn message() {
    let message_quit = Message::Quit;
    let message_move = Message::Move { x: 0, y: 0 };
    let message_write = Message::Write(String::from("message"));
    let message_change_color = Message::ChangeColor(1, 2, 3);
    println!("message_quit is: {:#?}", message_quit);
    println!("message_move is: {:#?}", message_move);
    println!("message_write is: {:#?}", message_write);
    println!("message_change_color is: {:#?}", message_change_color);
}
