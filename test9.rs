trait Foo {
    fn foo(&self);
    
    // default method
    fn bar(&self) {
        println!("we called bar.");
    }
}

//inheritance
trait FooBar : Foo {
    fn foobar(&self);
}

struct Animal;

impl Foo for Animal {
    fn foo(&self) {
        println!("foo");
    }
}

impl FooBar for Animal {
    fn foobar(&self) {
        println!("foobar");
    }
}

fn print_function<T: FooBar>(fun: T) {
    println!("{:?}, {:?}, {:?}",fun.foo(), fun.foobar(), fun.bar());
}

fn main() {
  let a = Animal;
  a.foo();
  a.foobar();
  print_function(a);
}
