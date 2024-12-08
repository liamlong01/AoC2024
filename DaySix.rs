use std::fs::File;
use std::io::prelude::*;

fn main() {
//open file
    let mut file = File::open("DaySix.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");
    
    let line_length = (contents.find("\n").expect("error finding line length") as i32)+1;
    println!("{}",line_length);

    let numLines = contents.lines().count() as i32;

    let mut total:i32 = 0;

    let mut position = IndexToCoords(contents.find("^").expect("error finding guard position")as i32, line_length);

     
   
 let mut dx = 0i32; // facing up
    let mut dy = -1i32;// facing up
    let mut iterations = 0;
    while InBounds(position, line_length, numLines)
    {
        let index = CalcIndex(position,line_length);
          
        let currentChar = contents.chars().nth(index); 
        if  currentChar.unwrap()!='X'
        {
            contents.replace_range(index..(index+1),"X");
            total+=1;
        }

        let mut test_position = AddTuples(position , (dx,dy));
        
        while InBounds(test_position,line_length,numLines)&&(contents.chars().nth(CalcIndex(test_position,line_length)).unwrap() == '#')
        {
            (dx,dy) = rotate90(dx,dy);
         
            test_position = AddTuples(position , (dx,dy));
        }

        position = test_position;
        
        //visual output
        if iterations%1000 == 0
        {
            println!("position: {:#?}, Total is {} state:\n{}",position,total, contents);
        }
        iterations+=1;

    }
    println!("Spaces visited: {}", total);

}

fn CalcIndex(position: (i32,i32),  line_length: i32) -> usize
{

    return (position.0+(line_length)*position.1) as usize;
}
fn IndexToCoords(index:i32, line_length:i32) ->(i32,i32)
{
    let i= index%(line_length);
    let j=index/(line_length);

    return (i,j);
}
fn InBounds(position: (i32,i32), line_length:i32, numLines:i32) ->bool
{

    return (position.0>=0)&&(position.0<line_length)&&(position.1>=0)&&(position.1<numLines);
}

fn rotate90(mut dx:i32, mut dy:i32) -> (i32,i32)
{
    if dx==0
    {
        dx = -dy;
        dy=0
    }
    else if dy == 0
    {
        dy=dx;
        dx=0
    }

    return (dx,dy)

}
fn AddTuples(tuple1: (i32,i32), tuple2: (i32,i32)) ->(i32,i32)
{
    return (tuple1.0+tuple2.0,tuple1.1+tuple2.1);
}