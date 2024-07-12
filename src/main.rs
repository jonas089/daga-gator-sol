pub mod client;
pub mod storage;
pub mod types;
use colored::*;

fn main() {
    println!(
        r#"
           .-._   _ _ _ _ _ _ _ _
.-''-.__.-'00  '-' ' ' ' ' ' ' ' '-.
'.___ '    .   .--_'-' '-' '-' _'-' '._
 V: V 'vv-'   '_   '.       .'  _..' '.'.
   '=.____.=_.--'   :_.__.__:_   '.   : :
           (((____.-'        '-.  /   : :
                             (((-'\ .' /
                           _____..'  .'
                          '-._____.-'"#
    );
    println!(
        "{}{} {}!",
        "Data".green(),
        "Gator".yellow(),
        "says Hello".blue()
    );
}
