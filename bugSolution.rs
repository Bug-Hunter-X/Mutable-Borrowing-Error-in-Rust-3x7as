fn main() {
    let mut x = 5;
    { //This is the solution
        let y = &mut x; 
        *y += 1; 
    }
    { //This is the solution
        let z = &mut x;
        *z += 1;
    }
    println!("{}", x);
}