use common::adv_io;

fn main() {
    let input = adv_io::read_input("d1/input");
    let result = d1::solution::sum_of_colibration_values(input);


    println!("{}", result);
}
