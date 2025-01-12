pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("none");
    let bit_count = num.to_string().len();
    num.to_string().as_str().chars().fold(0, |now, num_char| {
        let to_num = num_char.to_digit(10).unwrap();
        now + to_num.pow(bit_count as u32)
    }) == num
}

#[test]
fn my_test() {
    // check how to_digit work
    let num_1: u32 = 11;
    let num_2: u32 = 22;
    num_1.to_string().as_str().chars().for_each(|num_c| {
        println!("1 {:?}", num_c.to_digit(10).unwrap());
    });
    num_2.to_string().as_str().chars().for_each(|num_c| {
        println!("2 {:?}", num_c.to_digit(10).unwrap());
    });
}
