use std::fs::File;
use std::io::prelude::*;

fn main() {
//open file
    let mut file = File::open("DayTwo.dat").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Error reading file contents");
 
    //convert each line to array
    //
    let mut numsafe = 0;
    for line in contents.lines(){
        
        println!("{}, total: {}", line, numsafe);
       y let words_int: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if checkSafetyVector(&words_int, true){numsafe+=1;};
       
        

    }
    println!("{}",numsafe);
}

fn checkSafety(first: &i32, second: &i32) -> i32{
            let diff = second - first;
            

            //check if between +1 and +3
            if !(diff.abs() >=1  && diff.abs() <= 3) {
                return 0;
            }
            return diff.signum();
            

}

fn checkSafetyVector(words_int: &Vec<i32>, tolerate: bool) -> bool
{

       
        let mut safe = true;
        let mut dir = 0;

                                               
        let mut words_iter = words_int.iter();
        
        //check if safe
        //let mut words_iter = words_int.iter();
        println!("{:?}, {}", words_int, tolerate);
        let  mut first_num = words_iter.next().unwrap(); //skip first
        let  mut prev_num = words_iter.next().unwrap(); //skip first
        let mut test_vec = words_int.to_vec();
        
        let direction = checkSafety(first_num,prev_num);
        match direction{
            -1|1 => (),

                0=> {   
                    if tolerate{
                        test_vec = words_int.to_vec();
                        test_vec.remove(0);
                            
                        if checkSafetyVector(&test_vec, false){return true;}

                        test_vec = words_int.to_vec();
                        test_vec.remove(1);

                        if checkSafetyVector(&test_vec, false){return true;}
                    }
                    return false;
                },

 
                _=>println!("checkSafety returned unexpected value, {}",direction)
        }
        
        
        for (index, number) in words_iter.enumerate()
        {
            let cur_dir = checkSafety(prev_num, number);
           
            if cur_dir != direction 
            {   
                if !tolerate{return false;}  
                else
                {
                    
                    test_vec = words_int.to_vec();
                   // println!("about to remove {} from {}", test_vec[index], test_vec[0]);
                    test_vec.remove(index);
                    //println!("done remove");


                    if checkSafetyVector(&test_vec, false){return true;}

                    test_vec = words_int.to_vec();
                    if index+1 < test_vec.len(){
                    test_vec.remove(index+1);

                    if checkSafetyVector(&test_vec, false){return true;}
                    }

                    test_vec = words_int.to_vec();

                    if index + 2 < test_vec.len(){
                    test_vec.remove(index+2);

                    if checkSafetyVector(&test_vec, false){return true;}
                    }

                    return false;
                } 
        
                
            }
            prev_num = number;
        }
       return safe;
      
        
}
