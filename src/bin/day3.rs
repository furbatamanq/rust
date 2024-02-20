fn main(){
    let v1 = 251_u16 + 8;

    /* same as adding with safety */
    let v2 = u16::checked_add(251, 8).unwrap();
    
    println!("v1 = {:?}, v2 = {:?}",v1, v2)
}

