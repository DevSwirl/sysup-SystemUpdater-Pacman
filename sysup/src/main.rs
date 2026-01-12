
use std::io;
use std::process::Command;



fn main() {



    println!("
┏┓   •  ┓  ┏┓   ┳┳   ┓     
┗┓┓┏┏┓┏┓┃  ┗┓┓┏┏┃┃┏┓┏┫┏┓╋┏┓
┗┛┗┻┛┗┛ ┗  ┗┛┗┫┛┗┛┣┛┗┻┗┻┗┗ 
              ┛   ┛        ");






    println!("Press Enter to update your system!: ");

    let mut enter = String::new();


    io::stdin()
        .read_line(&mut enter)
        .expect("ERROR");

    println!("ILL GET CRACKING MY GUY!! NNEEEOOOOWWW");
 
command()
    
}


fn command() {



    Command::new("sudo")
    .arg("pacman")
    .arg("-Syu")
    .status()
    .expect("ERROR");


    Command::new("yay")
    .arg("-Syu")
    .status()
    .expect("ERROR");


    Command::new("sudo")
    .arg("journalctl")
    .arg("--vacuum-time=2weeks")
    .status()
    .expect("ERROR");





}
