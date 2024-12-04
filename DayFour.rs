use std::fs::File;
use std::io::prelude::*;

fn main() {
//open file
    let mut file = File::open("DayFour.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");
    
    let line_length = contents.find("\n").expect("error finding line length");
    let numLines = contents.lines().count();

    let mut total = 0;
    for i in 0..numLines{
        for j in 0..(line_length-1) //-1 to ignore \n
        {
            
            let n = j+(line_length+1)*i; //+1 to include /n in linelength
            let currentChar = contents.chars().nth(n).expect("error indexing contents as chars");


            if currentChar == 'A'
            {
                
                let mut value = searchMAS(n,line_length+1,&contents);
                //search all directions
                if value{total+=1;}

                println!("({},{}) : {}", i,j, total);
                            }

        }
    }
}


fn searchWord(n:usize,line_length:usize,content:&String, word:&str) -> u32
{
    
    let mut count:u32 = 0;
    
    let directions = [1i32, 0, -1];
    for dx in directions.iter(){
        for dy in directions.iter(){

            let mut found = true;
            let mut index:usize = n;
            for c in word.chars(){
               // println!("({},{}) index {}", dx,dy,index); 

                let _ = match content.chars().nth(index){
                    Some(val) if val==c => index = ((index as i32)+dx+dy* (line_length as i32)) as usize,
                    _ => {found = false; break;},
                };
               
                
    
                

            }
            if found{count+=1;}
        }
    }
    return count;

}

fn searchMAS(n:usize,line_length:usize,content:&String) -> bool
{
    
    let mut count:u32 = 0;
    
    let directions = [1i32, 0, -1];
    for dx in directions.iter(){
        for dy in directions.iter(){
            
            
            if (*dx==0i32)&&(*dy==0i32){continue;}

            if checkCross(n, *dx,*dy,content,line_length){return true;}
        }

    }
    return false;

}
fn checkCross(n:usize, dx:i32,dy:i32,content:&String,line_length:usize) -> bool
{
    if checkMAS(n,dx,dy,content, line_length)
    {
        if (dx!=0 && checkMAS(n, -dx,dy,content,line_length)) || (dy!=0 && checkMAS(n, dx,-dy,content,line_length))
        {return true;}
    }
    return false;
}

fn checkDirection(n:usize,dx:i32,dy:i32,content:&String,line_length:usize, cToLook:char) -> bool
{
    let mut index:usize = ((n as i32)+dx+dy* (line_length as i32)) as usize;

    let _ = match content.chars().nth(index)
    {
        Some(c) if c==cToLook => return true,
        _=>  return false,

    };
}

fn checkMAS(n:usize, dx:i32,dy:i32,content:&String,line_length:usize) -> bool
{
    if checkDirection(n, dx,dy, content, line_length,'M')
    {
        if checkDirection(n, -dx,-dy,content,line_length,'S')
        {
            
            //println!("found MAS {},{}",dx,dy);
            return true;
        }
    }    
    return false;
}



