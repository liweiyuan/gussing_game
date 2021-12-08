use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数组游戏!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    //println!("神秘数字； {}", secret_num);

    loop{
        println!("猜测一个数"); 

        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("无法读取行");

        let guess_num: u32 = match guess_num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数: {}", guess_num);
        
        match guess_num.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}
