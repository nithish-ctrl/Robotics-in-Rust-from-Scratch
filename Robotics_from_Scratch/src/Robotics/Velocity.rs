// Velocity & Static Forces in Manipulators 

fn main(){
    println!("This is working");

    // Lets use jacobian to find thetha 
    let mut xe;
    std::io::stdin()
    .read_line(&mut xe);

    let mut ye;
    std::io::stdin()
    .read_line(&mut ye);

    println!("Xe: {}", xe);
    println!("Ye: {}", ye);

    //Time for us to do matrix mutliplication 
    let mut jacobian = [[0.0; 2]; 2] = [[0, 0], [1, 3]];

    // thetha = (jacobian)^-1 * [xe ye]


}