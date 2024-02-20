fn main(){
    let s = String::from("Hello");
    
    takes_ownership(s);

    // println!("{:?}",s);
}


fn takes_ownership(some_text:String){
    println!("{:?}",some_text)
}