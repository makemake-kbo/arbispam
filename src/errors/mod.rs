pub mod errors {
    // Prints a simple explenation and description of args required
    // for the program to run
    pub fn print_help() {
        println!("ARBISPAM");
        println!("Command line arguments _*required*_:");
        println!("1) Path to CSV list of SKs");
        println!("2) Claim address");
        println!("3) Token address");
        println!("4) HTTP RPC");
    }
}