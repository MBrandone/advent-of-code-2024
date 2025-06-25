use std::io;

mod day_1;
use day_1::day_1;

mod day_2;
use day_2::day_2;

mod day_3;
use day_3::day_3;

mod day_4;
use day_4::day_4::day_4;

mod day_5;
use day_5::day_5::day_5;

mod day_6;
use day_6::day_6;

mod day_7;
mod day_8;
mod day_9;
mod day_10;

use crate::day_10::day_10;
use crate::day_7::day_7;
use crate::day_8::day_8;
use crate::day_9::day_9;
// Utiliser les workspaces
// https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html

#[tokio::main]
async fn main() -> Result<(),io::Error > {
    
    if false {
        let _ = day_1().await;
        let _ = day_2().await;
        let _ = day_4().await;
        let _ = day_5().await;
        let _ = day_6().await;
        let _ = day_7().await;
        let _ = day_8().await;
        let _ = day_9().await;
        let _ = day_10().await;
    }
    
    let _ = day_3().await;
    Ok(())
}

