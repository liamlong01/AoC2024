use std::fs::File;
use std::io::prelude::*;

fn main() {
//open file
    let mut file = File::open("DayThree.dat").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Error reading file contents");
    
    let mut sum = 0;
    
    let mut mainStrIter = contents.split("don't()");
    sum+=parseForMul(mainStrIter.next().unwrap()); //mul is active until first don't()


    for testString in mainStrIter  {
        println!("testString: {}", testString);
        let mut possibleStrings = testString.split("do()");
       
        //first substring follows a don't(), is inactive so skip
        possibleStrings.next();
        
        for candidate in possibleStrings{
            
            println!("after do: {}",candidate);
            sum+=parseForMul(candidate);
        }

        

    }

    
    println!("{}", sum);
}

fn parseForMul(sToParse: &str)->i32
{
    
    let mut sum = 0;
    for candidate in sToParse.split("mul(").collect::<Vec<_>>()
    {
            
        println!("{}",candidate);

        let numbers: Vec<_> = candidate.split(",").collect::<Vec<_>>();
        let num1 = numbers[0].parse::<i32>();
        match num1{
            Ok(ok) => (),
            Err(e) => continue, //not a number
        }
        

        let num2 = numbers[1].split(")").collect::<Vec<_>>()[0].parse::<i32>();
        match num2{
            Ok(ok) => (),
            Err(e) =>continue, //not a number
        }

        sum+= num1.unwrap()*num2.unwrap();
    }

    return sum;
}

