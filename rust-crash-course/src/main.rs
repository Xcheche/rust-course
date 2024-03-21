#![deny(clippy::all)]

fn main() {
    let first_name = "obi";

    let last_name = "Doe";
    let age = 32;
    let distance_in_km = 5.5f32;
    println!("The dis kilo is :\n{}", distance_in_km);

    println!(
        "Your fullname is :\n{}{} and you are: \n {}years old",
        first_name, last_name, age
    );
    // End of variable declaration

    // operator
    let distance1 = 40;
    let distance2 = 10;
    let distance3 = 20;
    let total_distance = distance1 + distance2 + distance3;

    println!("The total distance is:\n {}", total_distance);

    //variable shadowing

    // let data = "foo";
    // let data = 10;
    // println!("{}", data);

    // let data = "foo";
    // {
    //     let data = data.to_string();
    // }

    // println!("{}", data);
    println!("My age is {}", MY_AGE);

    //tuple

    let personal_data = (11, "john");
    let (age, name) = personal_data;
    let age = personal_data.1;

    println!("{} {}", age, name);
}
//constant
// const MY_AGE = 32;
const MY_AGE: i32 = 20;
