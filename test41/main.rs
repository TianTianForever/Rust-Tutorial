mod my_module;
use my_module::beautiful_gril as bg;
fn main() {
    my_module::beautiful_gril::function();
    bg::function();
    bg::indirect_access();
    //Module 'information' is private.
    //my_module::information::function();
}
