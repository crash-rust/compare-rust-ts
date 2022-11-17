mod arrays;
mod boxs;
mod enums;
mod lifetime;
mod options;
mod rcs;
mod results;
mod strings;
mod structs;
mod traits;
mod variable;

fn main() {
    variable::variable();
    enums::enums();
    strings::strings();
    options::options();
    arrays::arrays();
    results::results();
    structs::structs();
    traits::traits();
    lifetime::lifetime();
    boxs::boxs();
    rcs::rcs();
}
