use std::env;

fn strlen(string:String) -> usize
{
    let mut i:usize = 0;
    for _char in string.chars() {
        i += 1;
    }
    return i;
}

fn main()
{
    let mut argv:Vec<String> = env::args().collect();
    if argv.len() == 1
    {
        println!("* LOUD AND UNBEARABLE FEEDBACK NOISE *");
        return;
    }
    argv.remove(0);
    for string in argv
    {
        print!("{}", strlen(string.clone()));
        print!("{}", string.to_uppercase());
    }
    println!("");
}
