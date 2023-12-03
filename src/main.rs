use common::adv_io;

fn main() {
    // Day 1
    // let input = adv_io::read_input("d1/input");
    // let result1 = d1::solution1::sum_of_colibration_values(&input);

    // let result2 = d1::solution2::sum_of_colibration_values(&input);

    // println!("{}", result1);
    // println!("{}", result2);
    //

    // Day 2
    //
    let input = adv_io::read_input("d2/input");
    let res1 = d2::solution1::sum_of_possible_games_ids(&input);
    println!("{}", res1);

    let res2 = d2::solution2::sum_of_powers(&input);
    println!("{}", res2)

}
