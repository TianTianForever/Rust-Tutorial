struct A;
struct S(A);

struct SGen<T>(T);

fn ref_g(_s: S) {

}

fn gen_spec_t(_s: SGen<A>) {

}

fn gen_spec_i32(_s: SGen<i32>) {

}

fn gen_spec_f64(_s: SGen<f64>) {

}

fn generic<T> (_s: SGen<T>) {

}

fn main() {
    generic::<char>(SGen('g'));

    generic(SGen('l'));

    ref_g(S(A));

   gen_spec_t(SGen(A));

   gen_spec_i32(SGen(12));

   gen_spec_f64(SGen(1.111));
}
