mod impl_mio;
mod impl_futures;
use impl_mio::*;
use impl_futures::*;
extern crate mio;
extern crate futures;
extern crate futures_cpupool;


fn main() {
    const BIG_PRIME: u64 = 15485867;
    use_mio();
    
    // Synchronous version.
    if synchrony(BIG_PRIME) {
        println!("Synchonous version.");
        println!("Prime");
    } else {
        println!("Not prime");
    }

    // Asynchronous version.
    asynchrony(BIG_PRIME);
    asynchrony_v2(BIG_PRIME);
}
