struct Apple;

struct Fruit(Apple);

struct Greengrocers<T>(T);


fn gen_a(_s: Fruit) {

}

fn gen_spec_i32(_s: Greengrocers<i32>) {

}

fn gen_spec_f64(_s: Greengrocers<f64>) {

}

fn generic<T> (_s: Greengrocers<T>) {

}

fn main() {
    generic(Greengrocers('g'));

    generic::<char>(Greengrocers('l'));

    gen_a(Fruit(Apple));    
   
    gen_spec_i32(Greengrocers(12));
    
    gen_spec_f64(Greengrocers(1.111111));
}




