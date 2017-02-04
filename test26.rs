// 'MyU64' is a new name for 'u64'.
type MyU64 = u64;
type MyAnotherU64 = u64;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
type m_u64 = u64;
fn main() {
    // 'MyU64' = 'MyAnotherU64' = 'm_u64'.
    let number: MyU64 = 5 as m_u64;
    let number2: MyAnotherU64 = 10 as m_u64;
    // Note that type alises don't privide any extra type safety, because
    // aliases are not new types.
    println!("{} number + {} number2 = {}",
             number,
             number2,
             number + number2
            );
}
   
