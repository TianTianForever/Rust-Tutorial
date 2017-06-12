use std::marker::PhantomData;
// A Phantom type parameter is one that doesn's show up at runtime, 
//but is checked statically at compile time.

/*
struct PhantomDate<T: ?Size>;
*/

// A phantom tuple struct which is generic over A with hidden parameter B.
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over A whis hidden parameter B. 
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>
}

fn main() {
    
    // PhantomType specified as <i32, f64>.
    let tuple1: PhantomTuple<i32, f64> = PhantomTuple(1, PhantomData);
    
    // PhantomType specified as <i32, f32>.
    let tuple2: PhantomTuple<i32, f32> = PhantomTuple(1, PhantomData);

    // PhantomStruct specified as <i32, f64>.
    let struct1: PhantomStruct<i32, f64> = PhantomStruct {
        first: 1,
        phantom: PhantomData,
    };

    //  PhantomStruct specified as <i32, f32>.
    let struct2: PhantomStruct<i32, f32> = PhantomStruct {
        first: 1,
        phantom: PhantomData,
    };
    
    // Compile-time error! Type mismatch so these cannot be compared.
    // println!("tuple1 == tuple2: {:?}", tuple1 == tuple2);
    // println!("struct1 == struct2: {:?}", struct1 == struct2);
    let a = Some(2.2);
    println!("{:?}",a);
}
