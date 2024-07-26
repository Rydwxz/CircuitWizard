use bevy::prelude::VariableCurve;
mod lib;

fn main() {
    println!("{}", test(2));
}

fn test(mut variable: i32 ) -> f32 {
    if variable == 2 {return variable as f32;}
    else {
        variable += variable + 2;
        variable as f32
    }
}
