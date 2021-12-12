// public struct that takes 3 arguments
#[derive(Debug)] // declare this so Args can derive Debug then use Display for printing
pub struct Args {
    pub first_image: String,
    pub second_image: String,
    pub final_output: String,
}

fn get_nth_args(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

// Args returns new argument from user input
impl Args {
    // publicably accessible init function
    pub fn new() -> Self {
    Args {
        first_image: get_nth_args(1),
        second_image: get_nth_args(2),
        final_output: get_nth_args(3),
    }
}

}