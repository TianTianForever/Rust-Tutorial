fn z105_z106() {
    // from z105 to z106 of (18.06 23.44 24)
    let variable: f32 = 1.01 / 42.04;
    println!("{:?}", variable);
    let mut z: f32 = 62.75; // z105 into stone of top level.
    // z105 - z106 = 41.5
    for i in 0..43 {
        println!("from z105 to z106: {:?}M: {:?}", i, z);
        z -= variable;
    }
}

fn z106_z107() {
    // from z106 to z107 of (388.06 - 420.19 = 32)
    // 63.10 - 61.74 = 1.36
    let var: f32 = 0.0425;        // 1.36 / 32.0 = 0.0425
    let mut z106: f32 = 61.74;
    println!("{:?}", var);
    for i in 19..52 {
        println!("E1-{:?}: {:?}",i, z106);
        z106 += var;
    }
}
fn z107_z108() {
    // from z107 to z108 of (420.19 - 460.43 = 40.24)
    // 63.10 - 62.29 = 0.81
    let var: f32 = 0.81 / 40.24;
    let mut z107: f32 = 63.10;
    for i in 51..97 {
        println!("E1-{:?}: {:?}", i, z107);
        z107 -= var;
    }
}
fn z108_z109() {
    // from z108 to z109 of (460.43 - 497.5 = 37.07)
    // 62.29 - 62.77 = 0.48;
    let v: f32 = 0.48 / 37.07;
    let mut z108: f32 = 62.29;
    for i in 97..135 {
        println!("E-{:?}: {:?}", i, z108);
        z108 += v;
    }
}

fn z109_z110() {
    // from z109 to z110 of (497.5 - 585.42 = 87.92)
    // 62.77 - 65.97 = 3.2;
    let v: f32 = 3.2 / 87.92;
    let mut z109: f32 = 62.77;
    for i in 134..138 {
        println!("E1-{:?}: {:?}", i, z109);
        z109 += v;
    }
}
fn main() {

    z105_z106();
    //z106_z107();
    //z107_z108();
    //z108_z109();
    //z109_z110();
}
