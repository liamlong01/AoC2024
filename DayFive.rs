use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
//open file
    let mut file = File::open("DayFive_Part1.dat").expect("file not found");
    let mut contents = String::new();


    file.read_to_string(&mut contents).expect("Error reading file contents");

    let mut page_rules = HashMap::new();
    let mut linenum = 0;
    let mut line_iter = contents.lines();
    
    for line in line_iter.by_ref().into_iter()
    {
        linenum+=1;

        if line == ""
        {
            break; //done reading rules
        }

        let pages = line.split('|').collect::<Vec<_>>();

        println!("{},{:#?}, got {:#?}",linenum-1,line, pages);
        let first_page:u32 = pages[0].parse::<u32>().expect("error converting page number");

        let second_page:u32 = pages[1].parse::<u32>().expect("error converting page number");
        if !page_rules.contains_key(&first_page)
        {
            let mut page_list = Vec::new();
            page_list.push(second_page);
            page_rules.insert(first_page, page_list);
        } 
        else{
            let x: &mut Vec<u32> = page_rules.get_mut(&first_page).unwrap();
            x.push(second_page);
            
             
        }

    }



    println!("{:#?}", page_rules);
    let mut total = 0;
    for line in line_iter
    {
        
     let mut correct_order = true;
     let mut pages_int: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
     
     // rules are broken if page index comes after page in list 
     // reversing is convenient for checking these since broken rules have
     // indices that appear last in original vec (bad for sorting in part 2 though)
     pages_int.reverse(); 

     'outer: for (index, page_number) in pages_int.iter().enumerate()
     {
        
        //println!("page check {}", page_number);
        if page_rules.contains_key(&page_number)
        {
            for page_before in pages_int[(index+1)..].iter()
            {
                if page_rules[&page_number].contains(page_before)
                {
                    correct_order = false;
                    break 'outer;
                } 
            }
        }
     }

     if !correct_order
     {
        pages_int.sort_by(|a,b| compare_pages(a,b,page_rules.clone())); 
        println!("{:#?}", pages_int);
     
        total+=pages_int[(pages_int.len()-1)/2];
     }


    }
    println!("{}", total);
}

fn compare_pages(page1: &u32, page2: &u32, rules: HashMap<u32,Vec<u32>>)->Ordering
{
    
    if rules.contains_key(page2){
        if rules[page2].contains(page1)
        {
            return Ordering::Greater;
        }
    }
    if rules.contains_key(page1){
        if rules[page1].contains(page2)
        {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}
