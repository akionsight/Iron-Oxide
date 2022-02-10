fn main () {

    // conditional loops with while

    let mut time_to_liftoff = 3;

    let _res = while time_to_liftoff != 0 {
        println!("{}", time_to_liftoff);
        time_to_liftoff -= 1
    };

    println!("Liftoff");
    
    let rando_array = [10,20,30,40,50];

    for element in rando_array {
        println!("The value is {}", element);
    }

    for element in (1..5).rev(){
        println!("{}", element)
    }
    // this loop basically just executes code forever, no exceptions
    loop {
        println!("Yo")
    }
}