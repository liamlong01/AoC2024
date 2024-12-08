use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct GuardState
{
    pos: (i32,i32),
    facex: i32,
    facey: i32
}

fn main() {
//open file
    let mut file = File::open("DaySix.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");
    let numLines = contents.lines().count() as i32;
    let line_length = (contents.find("\n").expect("error finding line length") as i32)+1;
    let start = IndexToCoords(contents.find("^").expect("error finding guard position")as i32, line_length);

    

    let mut original_pos = getSeenSpaces(start, line_length, numLines, contents.clone());

    let mut total =0;
    
    for obstaclepos in original_pos.iter(){
        
          
            let mut newGarden = contents.clone();

       
            println!("Searching with {:#?}", obstaclepos);

            //don't replace guard
            if (newGarden.chars().nth(CalcIndex(*obstaclepos,line_length)).unwrap() == '^'){continue;}

            replaceChar(&mut newGarden, *obstaclepos,line_length,'#');

            if !FiniteRoute(newGarden.as_bytes(),start, line_length,numLines)
            {
                total+=1;

            }

        
    }
    println!("{}", total);

    

}
fn FiniteRoute(mut garden: &[u8] , start: (i32,i32), line_length:i32, numLines:i32) -> bool
{
    
 

    let mut position = start;


    let mut prev_states = HashSet::<GuardState>::new();

     
    let mut dx = 0i32; // facing up
    let mut dy = -1i32;// facing up


    while InBounds(position, line_length, numLines)
    {
        
        let mut test_position = AddTuples(position , (dx,dy));

        
        while InBounds(test_position,line_length,numLines)&&(garden[CalcIndex(test_position,line_length)] == b'#')
        {
            

            (dx,dy) = rotate90(dx,dy);
         
            test_position = AddTuples(position , (dx,dy));
            
            let state = GuardState{pos:test_position,facex:dx,facey:dy};
            if prev_states.contains(&state)
            {
                return false;
            }
            prev_states.insert( state );



        }
        
       
        position = test_position;

    }

    return true;

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

fn replaceChar(garden: &mut String, position: (i32,i32), line_length:i32, character:char)
{
     let index = CalcIndex(position,line_length);
          
    let currentChar = garden.chars().nth(index);  
    garden.replace_range(index..(index+1),&character.to_string());
  
}

fn getSeenSpaces(mut position: (i32,i32), line_length:i32, numLines:i32, mut contents: String) -> HashSet::<(i32,i32)>
{
    let mut positions_seen = HashSet::<(i32,i32)>::new();

    let mut dx = 0i32; // facing up
    let mut dy = -1i32;// facing up
    while InBounds(position, line_length, numLines)
    {
        positions_seen.insert(position);
        let index = CalcIndex(position,line_length);
          
        let currentChar = contents.chars().nth(index); 
        if  currentChar.unwrap()!='X'
        {
            contents.replace_range(index..(index+1),"X");
        }

        let mut test_position = AddTuples(position , (dx,dy));
        
        while InBounds(test_position,line_length,numLines)&&(contents.chars().nth(CalcIndex(test_position,line_length)).unwrap() == '#')
        {
            (dx,dy) = rotate90(dx,dy);
         
            test_position = AddTuples(position , (dx,dy));
        }

        position = test_position;
        

    }
    return positions_seen;
}