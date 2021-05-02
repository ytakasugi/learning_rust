#![allow(unreachable_code)]
#![allow(dead_code)]

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("The number is 3");
            
            continue;
        } 

        println!("{}", count);

        if count == 5 {
            println!("The number is 5");
            
            break;
        }
    }

    /*'outer: loop {
        println!("Entered the outer loop");
        
        'inner: loop {
            println!("Entered the inner loop");
            // 内側のループのみ抜ける
            //break;

            // 外側にループを抜ける
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");*/

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    //assert_eq!(result, 20);
    println!("{}", result);



}
