use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
//open file
    let mut file = File::open("DayEight.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");

    let line_length = contents.find('\r').expect("Error finding line length"); //newlines are \r\n
    let num_lines = contents.lines().count();

    let mut antenna_locs = HashMap::new();

    for (index, antenna) in contents.chars().enumerate()
    {
        let coords = IndexToCoords(index,line_length); //(i32,i32)
        let unwanted_chars = vec!['.','\r','\n'];
        if unwanted_chars.contains(&antenna){continue;}
        if !antenna_locs.contains_key(&antenna){
            let positions = vec![coords];
            antenna_locs.insert(antenna, positions);
        }
        else
        {
            let x: &mut Vec<(i32,i32)> = antenna_locs.get_mut(&antenna).expect("error reading hash map");
            x.push(coords);
        }

    }
    println!("{:#?}", antenna_locs); 

    let mut seen = Vec::new();
    for (key,val) in antenna_locs.iter()
    {
        if val.len() == 1 {continue;}

        for val1 in val.iter()
        {
            for val2 in val.iter() //need all possible pairs so two loops
            {
                if val1==val2 {continue;}

                let mut diff = SubtractTuples(val1, val2);
                let gcd = greatest_common_divisor(diff.0.abs(),diff.1.abs());

                // get smallest possible value for diff
                diff = ScalarIntegerDivision(gcd, diff); 
                
                

                //check positive direction
            
                let mut test_pos = val1.clone();
                while InBounds( &test_pos, line_length, num_lines)
                {
                    if !seen.contains(&test_pos)
                    {
                        seen.push(test_pos);
                    }
                    
                    
                    test_pos = AddTuples(&test_pos, &diff);
                }

                //check negative direction
                test_pos = val1.clone(); // there are no nodes in bet
                while InBounds(&test_pos,line_length, num_lines)
                {
                    if !seen.contains(&test_pos)
                    {
                        seen.push(test_pos);
                    }
                    test_pos = SubtractTuples(&test_pos, &diff);
                }

             
            }


        }
    }
    seen.sort_by(|a,b| a.1.cmp(&b.1));
    println!("{:#?}. {}", seen, seen.len());
    

}

fn CalcIndex(position: (i32,i32),  line_length: i32) -> i32
{

    return (position.0+(line_length+2)*position.1) as i32;
}
fn IndexToCoords(index:usize, line_length:usize) ->(i32,i32)
{
    // line_length should be length that ignores \r\n characters
    // add 2 to get actual line_length
    let i= index%(line_length+2);
    let j=index/(line_length+2);

    return (i as i32,j as i32);
}
fn AddTuples(tuple1: &(i32,i32), tuple2: &(i32,i32)) ->(i32,i32)
{
    return (tuple1.0+tuple2.0,tuple1.1+tuple2.1);
}
fn SubtractTuples(tuple1: &(i32,i32), tuple2: &(i32,i32)) -> (i32,i32)
{
    return (tuple1.0-tuple2.0,tuple1.1-tuple2.1);
}
fn InBounds(position: &(i32,i32), line_length:usize, numLines:usize) ->bool
{

    return (position.0>=0)&&(position.0<line_length as i32)&&(position.1>=0)&&(position.1<numLines as i32);
}
fn ScalarMultiple(scale: i32, math_vec: (i32,i32))-> (i32, i32)
{
    return (math_vec.0*scale, math_vec.1*scale);
}
fn ScalarIntegerDivision(scale: i32, math_vec: (i32,i32))-> (i32, i32)
{
    return (math_vec.0/scale, math_vec.1/scale);
}

fn greatest_common_divisor(mut n: i32, mut m: i32) -> i32 
{
  
  while m != 0 {

    if m < n {
      let temp = n;
      n=m;
      m=temp;
    }
    m %= n;
  }
  return n
}