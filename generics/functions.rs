struct A;          // Concrete type 'A'.
struct S(A);       // Conctete type 'A'.
struct SGen<T>(T); // Generics type 'SGen'.

fn reg_fn(_s: S) {

}

fn gen_spec_t(_s: SGen<A>) {

}

fn gen_spec_i32(_s: SGen<i32>) {

}

fn gen_spec_u64(_s: SGen<u64>) {

}

fn generic<T>(_s: SGen<T>) {


}

fn main() {
    reg_fn(S(A));

    gen_spec_t(SGen(A));
   
    gen_spec_i32(SGen(12));

    gen_spec_u64(SGen(22));

    generic::<char>(SGen('g'));

    generic(SGen('l'));
}
