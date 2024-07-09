mod prob_26_50;
mod probs_1_25;

fn main() {
    let config = "26-50";

    match config {
        "1-25" => probs_1_25::main(),
        "26-50" => prob_26_50::main(),
        _ => println!("Incorrect config"),
    };
}
