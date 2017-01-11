use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

static CONTENT: &'static str =
"Can you make the tea,sam? Yes, of course I can,Peeny.
Is there any water in this kettle? Yes,there is. Where's the tea?It's over there.
behind the teapot. Can you see it? I can see the teapot, but I can't see the tea";

fn main() {
    //Create a path to the desired file.
    let path = Path::new("out/content.txt");
    //Display path
    let display = path.display();
 
    // Open a file in the write-only mode,returns the io::Result<Flie> type.
    let mut file = match File::create(&path) {
        Err(why) => panic!("could't create {}: {}",display, why.description()),
        Ok(file) => file,
    };
 
    //Write the 'CONTENT' string to 'file', returns 'io::Result<()>'
    match file.write_all(CONTENT.as_bytes()) {
        Err(why) => { 
            panic!("Could't write to {}: {}",display, why.description())
        },
        Ok(_) => println!("successfully wrote to {}",display),
    }
    
}
