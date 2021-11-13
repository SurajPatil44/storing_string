use storing_string;

fn main() {
    let mut t = storing_string::trinode::new();

    t.triinsert("first");
    t.triinsert("second");

    t.print_words();
}