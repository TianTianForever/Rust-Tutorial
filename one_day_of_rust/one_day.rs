#![allow(unused_variables)]
// A concrete type A.
struct A;

// Concrete type 'DefineType'.
struct DefineType(A);


// 'DefineTypeGen' is a generic type.
struct DefineTypeGen<T>(T);


// Define a function 'take_definetype' that takes an argument 'd' of type 'DefineType'.
fn take_definetype(d: DefineType) {

}

// Define a function 'gen_spec_d' that takes an argument 'g' of type 'DefineTypeGen'.
fn gen_spec_d(g: DefineTypeGen<A>) {

}

// Define a function 'generic_specified' that takes an argument 'g' of type 'DefineTypeGen'.
fn generic_specify_d(g: DefineTypeGen<DefineType>) {

}

fn gen_spec_i32(g: DefineTypeGen<i32>) { }

fn gen_spec_f64(g: DefineTypeGen<f64>) { }

fn gen_spec_char(g: DefineTypeGen<char>) { }

fn gen_spec_string(g: DefineTypeGen<&str>) { }

//define a function 'generic' that tackes an argument 'g' of type 'DefineTypeGen', this function is generic.
fn generic<T>(g: DefineTypeGen<T>) {

}

fn main() {
    // 'DefineTypeGen' is concrete and explicitly takes 'A'.
    let a = DefineType(A);

    //Create a variable '_char' of type 'DefineTypeGen<char>'
    // here, 'DefineTypeGen' has a type paramer explicitly specified.
    let _char: DefineTypeGen<char> = DefineTypeGen('a');

    // u32 type 
    let _u32: DefineTypeGen<u32> = DefineTypeGen(222);
    
    // i32 type
    let _i32: DefineTypeGen<i32> = DefineTypeGen(222);
 
    // String type
    let _string: DefineTypeGen<&str> = DefineTypeGen("explicitly");
 
    // f64 type 
    let _f64: DefineTypeGen<f64> = DefineTypeGen(6.0);
    


    // 'DefinetypeGen' can also has a type paramer  implicitly specified.
    // Uses 'i32'.
    let _integer = DefineTypeGen(6);
 
    // Uses 'A' define at the top.
    let _a = DefineTypeGen(A);

    // Uses 'float'.
    let _float = DefineTypeGen(6.0);

    // Uses 'String literal'. Note that this type is allocated on the stack.
    let _string = DefineTypeGen("This type is allocated on the heap ");

    // Uses 'String'. Note that this type is allocated on the heap and as such is able to store an amount of 
    // text that is unkonwn to us at compile time.
    let s = String::from("this type is allocated on the heap");
    let _heap = DefineTypeGen(s);   
   
    // Uses 'char'
    let _char = DefineTypeGen('c');


    // Using the non-generic functions.
    take_definetype(DefineType(A));

    gen_spec_d(DefineTypeGen(A));
   
    generic_specify_d(DefineTypeGen(DefineType(A)));
    
    //gen_spec_i32(_i32);
    //gen_spec_f64(_float);
    //gen_spec_char(_char);
    //gen_spec_string(_string);

    generic(_i32);
    generic(_float);
    generic(_char);
    generic(_string);
}
