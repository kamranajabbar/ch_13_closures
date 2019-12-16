pub fn run() {
    let x = 4;
    let equal_to_x = |z| z == x;

    // Work fine as both values are equal
    assert!(equal_to_x(6), "My Code Error");

    //Will be panic due to un equalilty
    assert!(equal_to_x(6), "My Code Error");
}