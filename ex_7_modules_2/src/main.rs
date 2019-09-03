enum TrafficLight {
    Red,
    Orange,
    Green
}

use TrafficLight::{Red, Orange};
//use TrafficLight::*

fn main() {

    let red = Red;
    let orange = Orange;
    let green = TrafficLight::Green;
}