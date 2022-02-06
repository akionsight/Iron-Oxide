fn main() {
    // okay, rust if statements; good luck!

    let num = 6;

    if num > 5 {
        println!("Number greater than 5");
    } else {
        println!("Number less than 5")
    }

    // else if statements

    let rando_manbo = 12;

    if rando_manbo % 2 == 0 {
        println!("Number divisible by 2")
    } else if rando_manbo % 3 == 0 {
        println!("Number divisible by 3")
    } if rando_manbo % 4 == 0 {
        println!("Number divisible by 4")
    } else {
        println!("Number not divisible by 2, 3 or 4")
    }

    // single liner if statments

    let condition = true;

    if condition { println!("True") } else { println!("False") }

    // ooooooo, fancy (in *that* tone)
}
