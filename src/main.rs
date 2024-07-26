use bevy::prelude::VariableCurve;
mod lib;

fn main() {
    println!("{}", test(23));
}

fn test(variable:i32 ) -> f32 {
    if variable == 23 {return variable as f32;}
    else {variable + 23;
    variable as f32}
}
