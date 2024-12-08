use std::fs::File;
use std::io::prelude::*;

fn main() {
//open file
    let mut file = File::open("DaySeven.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");
    let mut sum:u64 = 0;


    for line in contents.lines()
    {
        println!("{}",line);
        let split_str: Vec<_> = line.split([':',' ']).collect();

        let target = split_str[0].parse::<u64>().unwrap();

        let num_list: Vec<u64> = split_str[2..].iter().map(|x| x.parse::<u64>().unwrap()).collect();

        println!("{}:{:#?}", target, num_list);

        let operator_spaces = num_list.len()as u64 - 1;

        
        let mut found = false;
        for i in 0..(2u64.pow(operator_spaces as u32)as usize)
        {
            let mut result = num_list[0];
            let mut test = i;
            for j in 0..(operator_spaces as usize)
            {
                if (test%2 == 1) 
                {
                    result = result * num_list[j+1];
                }
                else {
                    result = result + num_list[j+1];
                }
                test/=2;
            }

            if result==target{found = true; break;}

            // println!("result at i: {} at {}", result, i);

        }
        if found {sum+=target;}
       
    }
    println!("{}", sum);
}