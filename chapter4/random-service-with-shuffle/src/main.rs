extern crate futures;
extern crate hyper;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::ops::Range;

#[derive(Deserialize)]
struct RngResponse {
    value: f64,
}

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
enum RngRequest {
    Uniform {
        range: Range<i32>
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    }
}

fn main() {
    println!("Hello, world!");
}
