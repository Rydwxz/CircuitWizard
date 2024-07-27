use bevy::prelude::*;

mod lib;
fn main() {
    println!("Welcome, Wizard, to the Wonderful Wagic Simulator!");
    println!("{}", test1(4));
    println!("{}", test2(4));
    let arg = 2;
    println!("{}", test1(arg));
    println!("{}", test2(arg));
}

fn test1(param: i32) -> i32 {
    if param == 2 { return param; }
    // can't do this:
    // param += 2;
    // param is not modified or returned: new value (param + 2) is constructed:
    return param + 2;
}
// keyword 'mut' makes the param 'mutable' i.e. can be modified
fn test2(mut param: i32 ) -> f32 {
    if param == 2 {
        param += 67;
    // using keyword 'as' is to be avoided: the type system is strict for a reason
        return param as f32;
    }
    else {
        param = 420;
        return param as f32;
    }
}
