pub fn run() {
    let closures_var = |x: i32| { 
        println!("{}", x);
    };

    closures_var(101);
}