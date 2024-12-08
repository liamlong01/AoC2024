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
       
        let split_str: Vec<_> = line.split([':',' ']).collect();

        let target = split_str[0].parse::<u64>().unwrap();

        //split_str[1] is always "" (from ": " sequence in each line), we can ignore

        let num_list: Vec<u64> = split_str[2..].iter().map(|x| x.parse::<u64>().unwrap()).collect();

       

        let operator_spaces = num_list.len()as u64 - 1;

        
        let mut found = false;
        for i in 0..(3u64.pow(operator_spaces as u32)as usize)
        {
            let mut result = num_list[0];
            let mut test = i;
            for j in 0..(operator_spaces as usize)
            {

                if (test%3 == 2) 
                {
                    result = result * num_list[j+1];
                }
                else if (test%3==1){
                    result = result + num_list[j+1];
                }
                else //concatenation
                {
                    let mut exponent_ten = 0;
                    let test_value = num_list[j+1];
                    while  test_value/10u64.pow(exponent_ten as u32) > 0
                    {
                        exponent_ten+=1;
                    }

                    result = result * 10u64.pow(exponent_ten as u32) + test_value;
                }
                if result>target {break;} //all operations increase result, so impossible once here
                test/=3;

            }

            if result==target{found = true; break;}

            //println!("result at i: {} at {}", result, i);

        }
        if found {sum+=target;}
       
    }
    println!("{}", sum);
}