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
fn test2(mut variable: i32 ) -> f32 {
    if variable == 2 {return variable as f32;}
    else {
        variable += variable + 2;
        variable as f32
    }
}
