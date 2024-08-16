//use std::env;
use std::str;
use std::fs;
fn main() {
    
    const IS_PART_ONE:bool = false;

    let file_path = "./final.input";
    println!("File path {file_path}");

    let input = fs::read_to_string(file_path).expect("Should have been able to read file"); 
    //println!("With text:\n{input}");

    if IS_PART_ONE {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        let mut total_sum =0;
        for line in lines {
            //println!("the value is: {line}");
            //get first and last digit
            let mut first_digit: char = '\0';
            let mut last_digit: char = ' ';
            for c in line.chars(){
                if c.is_numeric(){
                    if first_digit == '\0' {
                        first_digit = c;
                        last_digit = c;
                    }else{
                        last_digit = c;
                    }
                } 
            }
            let mut result = String::new(); 
            result.push_str(&first_digit.to_string());
            result.push_str(&last_digit.to_string());
            println!("Result inner: {result}");
            if !result.trim().is_empty() {
                let parsed_result:i32 = result.trim().parse().expect("oops");
                total_sum += parsed_result;
            }
       
        }
        println!("Result: {total_sum}"); 
    }else{
        let c = 'o';
        match c {
            'c' => println!("two"),
            'o' => {
                println!("one");
                if(source_string.len() <= "one".len()){

                }
            },
            _ => println!("else"),
        }


    }
}

fn is_overflow_safe(source_string:&str,size:usize ) -> bool{
    source_string.len() <= size
}
