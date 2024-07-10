mod prob_1_25;
mod prob_26_50;
mod prob_51_75;

fn main() {
    let config = "51-75";

    match config {
        "1-25" => prob_1_25::main(),
        "26-50" => prob_26_50::main(),
        "51-75" => prob_51_75::main(),
        _ => println!("Incorrect config"),
    };
}
