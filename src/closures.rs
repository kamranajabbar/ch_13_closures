pub fn run() {
    let closures_var = | | { 
        println!("I am using anonymous function called closures");
    };

    closures_var();
}