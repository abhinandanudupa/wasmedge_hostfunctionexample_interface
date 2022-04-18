use wasmedge_hostfunctionexample_interface::*;

fn main() {
    println!("Initialising an vector with {:?} randomn numbers.", 10u32);
    initialise(10u32);

    // println!("Here is the vector:");
    print();

    println!("\nSorting the vector using insertion sort...");
    sort();
    println!("\nSorting finished.");

    println!("\nThe sorted vector is:");
    print();
}
