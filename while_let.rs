fn main() {
    //Make 'optional' of type OPtion<i32>
    let mut optional = Some(0);
 
    // Repeatedly try this test.
    loop {
        match optional {
            // if 'optional' destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("'i' is {:?}, try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => { break; }
       }
    }

    // Make 'optional1' of type Option<i32>
    let mut optional1 = Some(0);
    
    // This reads: 'while' 'let' destructures 'optional1' into Some(i),
    // evaluate the block ('{}'). Else 'brak'.
    while let Some(i) = optional1 {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional1 = None;
        } else {
            println!("'i' is '{:?}', Try again.", i);
            optional1 = Some( i + 1);
        }
    }
    
}
