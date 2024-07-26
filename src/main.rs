use bevy::prelude::*;

mod lib;
fn main() {
    println!("{}", test1(4));
    println!("{}", test2(4));
}

fn test1(param: i32) -> i32 {
    if param == 2 { return param; }
    // can't do this:
    // param += 2;
    // param is not modified or returned: new value (param +2) is constructed:
    return param + 2;
}
// mut makes the param
fn test2(mut param: i32 ) -> f32 {
    if param == 2 {return param as f32;}
    else {
        param += param + 2;
        param as f32
    }
}
