use common::adv_io;

fn main() {
    let input = adv_io::read_input("d1/input");
    let result1 = d1::solution1::sum_of_colibration_values(&input);

    let result2 = d1::solution2::sum_of_colibration_values(&input);

    println!("{}", result1);
    println!("{}", result2);
}
