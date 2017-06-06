use std::fmt;
// Manually implementing 'Debug' and 'Display'.

// pub trait Debug {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;  
// }

pub struct User {
    pub email: String,
    pub username: String,
    pub active: bool,
    pub sign_in_count: u64,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,"User {{email: {}, usernmae: {}, active: {}, sign_in_count: {} }}",
            self.email, self.username, self.active, self.sign_in_count,
        )
    }
}

// pub trait Display {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
// }

impl fmt::Display for User {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f,"({}, {}, {}, {})",
         self.email, self.username, self.active, self.sign_in_count
         )
     }
}
