fn main() {
    // Make 'optional' of type 'Option<i32>'.
    let optional = Some(8);
    match optional {
        Some(i) => {
            println!("This is a really long string and '{:?}'",i);
            // ^ Needed 2 indentations just so we could destructure.
            // 'i' from the option.
        },
        _ => {},
    }; 
 
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    // The 'if let' construct reads:"if 'let' destructures 'number' into"
    // 'Some(i)', evaluate the block('{}').
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("matched: {:?}!", i);
    } else {
        // Destructure failed, Change to the failure case.
        println!("Don't match a number. Let's go with a letter!");
    };
    
    // Provide an altered failing condition.
    let i_like_letters = false;
    
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluate an 'else if' condition to see 
        //if the alternate failure branch should be taken:
    } else if i_like_letters { 
        println!("Didn't match a number, let's go to with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like lerrers, let's go with an emoticon :)!");
    }
}
