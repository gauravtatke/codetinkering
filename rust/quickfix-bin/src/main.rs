mod message;
mod quickfix_errors;
#[allow(unused_imports)]
mod types;

use crate::message::*;
use crate::types::*;

fn main() {
    let mut msg = Message::new();
    msg.header_mut().set_string(8, "FIX4.3".to_string());
    msg.header_mut().set_string(49, "Gaurav".to_string());
    msg.header_mut().set_string(56, "Tatke".to_string());

    msg.body_mut().set_int(34, 8765);
    msg.body_mut().set_float(44, 1.87856);
    msg.body_mut().set_bool(654, true);
    msg.body_mut().set_char(54, 'b');
    msg.body_mut().set_string(1, "BOX_AccId".to_string());

    msg.trailer_mut().set_int(10, 101);
    println!("{}", msg);
}
