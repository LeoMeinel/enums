/*
 * File: message.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
