/* concept about the borowing */
#[warn(unused_variables)]
fn main(){

    let mut name: String = String::from("Sandesh");

    let name2: &mut String = &mut name;


    println!("{:p}",name2);

    let x: i32 = 43;
    let y: &i32 = &x;

    println!("{}", *y);

    assert_eq!(x, *y);
}
