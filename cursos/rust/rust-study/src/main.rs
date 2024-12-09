// cargo watch -x run
//mod a1_ssemantics;
//mod a2_ssemantics;
//mod a3_ssemantics;
//mod a4_ssemantics;
//mod a5_ssemantics;
//mod hr;

fn main() {
    //a1_ssemantics::a1();
    //a2_ssemantics::a2();
    //a3_ssemantics::a3();
    //a4_ssemantics::a4();
    //a5_ssemantics::a5();
    //hr::hackerrank();
    name();

    println!("{:?}", name("Mashu", "kyrielight"));

    let result: String = name("Mashu", "kyrielight");
    println!("{:?}", result);
}

fn name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);
    return full_name;
}
