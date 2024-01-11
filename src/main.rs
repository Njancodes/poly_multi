use std::io;

fn main() {

    //Gather 2 polynomials into 2 arrays -DONE
    
    //Iterate through each array's element to multiply each other - DONE
    //Put the value in a new array - DONE

    let mut deg1 = String::new();
    let mut deg2 = String::new();

    println!("Enter the highest degree for your first polynomial: ");
    io::stdin().read_line(&mut deg1).expect("Something happened to taking input");

    println!("Enter the highest degree for your second polynomial: ");
    io::stdin().read_line(&mut deg2).expect("Something happened to taking input");

    let deg1:i32 = deg1.trim().parse().expect("Please enter a number next time");

    let deg2:i32 = deg2.trim().parse().expect("Please enter a number next time");

    let mut a:Vec<i32> = Vec::new();
    let mut b:Vec<i32> = Vec::new();

    for i in 0..deg1+1{
        println!("Enter the coefficient of the first polynomial's x^{}",i);
        let mut coeff = String::new();
        io::stdin().read_line(&mut coeff).expect("Something happened to taking input");
        let coeff:i32 = coeff.trim().parse().expect("Input a number please");
        a.push(coeff);
    }

    for i in 0..deg2+1{
        let mut coeff = String::new();
        println!("Enter the coefficient of the second polynomial's x^{}",i);
        io::stdin().read_line(&mut coeff).expect("Something happened to taking input");
        let coeff:i32 = coeff.trim().parse().expect("Input a number please");
        b.push(coeff);
    }

    let mut c:Vec<i32> = Vec::new();
    let deg3 = deg1 + deg2;

    for _i in 0..deg3+1{
        c.push(0);
    }

    for i in 0..(deg1+1) as usize{
        for j in 0..(deg2+1) as usize{
            if a[i] != 0 && b[j] != 0{
                c[i+j] += a[i] * b[j];
            }

        }
    }

    for i in (0..(deg3+1)).rev(){

        if i == 0{
            print!("{}x^{}",c[i as usize],i);
        }else{
            print!("{}x^{} + ",c[i as usize],i);
        }
    }

}
