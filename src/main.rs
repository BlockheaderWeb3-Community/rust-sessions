//mod strings;
mod float;
mod signed;
mod string;
mod unsigned;
mod user_book_constructor;
mod user_struct;

fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
    user_struct::user_registry();
    user_book_constructor::book_new();
}
