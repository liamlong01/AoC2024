use std::fs::File;
use std::io::prelude::*;


fn main() {
//open file

    let mut file = File::open("DayOne.dat").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Error reading file contents");
    
       
    let mut numbersOne: [i32; 1000] = [0; 1000];
    let mut numbersTwo: [i32; 1000] = [0; 1000];
    
    let mut index = 0;
    for line in contents.lines(){
        let words: Vec<&str> = line.split_whitespace().collect();
         
        numbersOne[index] = words[0].parse::<i32>().unwrap();
        numbersTwo[index] = words[1].parse::<i32>().unwrap();
        index += 1;
    }
    
// sorting
    numbersOne.sort();
    numbersTwo.sort();
    
    let mut sum = 0;
    for i in 0..1000{
        let num: i32 = numbersOne[i];
        for j in 0..1000{
            if numbersTwo[j] == num {
                sum+=num;
            }
        }
       
    }
    println!("{}",sum);


}
//open file


//create two lists (split)

//sort lists

// subtract lists

//sum
