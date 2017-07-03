use std::cell::Cell;
use std::collections::HashMap;
mod pointer;
use pointer as SmartPoint;

fn main() {
    let Tom = SmartPoint::Person {
        age: Cell::new(23),
        name: Some(String::from("Tom")),
        height: Box::new(170.0),
        weight: Box::new(60.0),
        sex: String::from("male"), 
    };
    println!("Tom information: {:?}", Tom);
    assert_eq!(Cell::new(23), *Tom);
    Tom.age.set(24);
    println!("Modify Tom information: {:?}", Tom);
    
    let mut accounts: SmartPoint::Accounts = HashMap::new();
    let account = SmartPoint::Account {
        username: "tiantianforever",
        password: "password1234567",
    };
    let account_info = SmartPoint::AccountInfo {
        name: "Tiantian",
        email: "someone@email@gmail.com",
    };
    accounts.insert(account, account_info);
    SmartPoint::try_logon(&accounts, "Tiantianforever", "password1234567");
    SmartPoint::try_logon(&accounts, "tiantianforever", "password1234567"); 
} 
