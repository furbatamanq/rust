
fn main(){
//   match_number()
// print_number(5)
}

/* match */
#[allow(dead_code)]
fn match_number(){
    let number:i32 = 50;

    match number{
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is greater than one two and three")
    }
}

#[allow(dead_code)]
fn match_day(){
    let day: &str = "nigga";
    match day {
        "sunday" =>{ 
            let  day_num:i32 = 1;
            println!("Today is {day} the {day_num} of week")},
        "monday" => println!("tuesday"),
        "tuesday" =>{
            println!("tuesday")
        }
        "wednesday" => {
            println!("wednesday")
        }
        "thursday" => {
            println!("thursday");
        }
        "friday" =>{
            println!("friday");
        }
        "saturday" =>{
            println!("saturday");
        } 
        _ => {
            println!("put correct input man");
        }
    }
}

/* loop */
#[allow(dead_code)]
fn print_number(num:i32){
    let mut i:i32 = 1;

    loop {
        println!("{:?}",i);

        if i == num {
            break;
        }

        i+=1;
    }
}