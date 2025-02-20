use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn add(i:u32,j:u32)->u32{
    println!("i:{i},j:{j}");
    i+j
}
fn test(num:u32)
{

    let x = {
        let y = 3;
        y+2
    };
    if(num + x) > 10 {
        println!("num+x > 10,x == {x}");
    }else{
        println!("num+x <= 10,x == {x}");
    }

    let condition = x > 3;
    let num = if condition{100} else{200} ;
    println!("num <= 10,num == {num}");
}

fn loop_func()
{
    let mut cnt = 10;
    let mut cnt2 = 88;
    'label: loop{
        cnt -=1;
        println!("cnt:{cnt}");       
        if cnt > 0{        
            loop{
                cnt2 -=1;
                println!("cnt2:{cnt2}");
                  
                if cnt2 < 58
                {
                    let mut c = 10;
                    while c>0
                    {
                        c-=2;
                        println!("c:{c}");
                    }
                    break 'label;
                }
                else if cnt2 ==  85
                {
                    cnt2 = 60;
                } 
                else
                {
                    break;
                }
            } 
        }else{

            break;
        }
    }
}

fn main() {
    loop_func();
    test(2);
    println!("Hello, world!");
    println!("Guess the number!");

    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=99);
    let mut guess = String::new();

    let x = "6789";
    const DEFAULT_NUM:u32 = 10;
    println!("x is {}",x);  
    let x = x.len();
    println!("x is {}",x);
    println!("y is {}",DEFAULT_NUM);
    let i:i8 = -2;
    let j:u16 = 65535;
    let k:f32 = 2.5;
    let l:bool = true;
    let m:char = 'a';
    let n:String = String::from("hello");
    let o:isize = 100;
    let p:char = '💕';



    let temp:(i32,char,u16) = (100,'💕',65535);
    let mut buf:[u32; 5] = [1,2,3,4,5];
    let buf2:[u32;5] = [10;5];
    buf[0] = buf2[1];
    
    buf[0] = add(buf[0],buf[1]);
    //let o:Vec<i32> = vec![1,2,3,4,5];
    //let p:HashMap<i32,i32> = HashMap::new();

    println!("i is {}",i);
    println!("j is {}",j);
    println!("k is {}",k);
    println!("l is {}",l);
    println!("m is {}",m);
    println!("n is {}",n);

    println!("o is {}",o);
    println!("p is {}",p);
    println!("temp is {:#?}",temp);
    println!("temp.0 is {}",temp.0);
    println!("temp.1 is {}",temp.1);
    println!("temp.2 is {}",temp.2);
    println!("buf = {:?}",buf);
    println!("buf = {:?}",buf2);
    //println!("o is {}",o);
    //println!("p is {}",p);
    for elem in buf{
        println!("elem:{elem}");
    }

    loop{
            guess.clear();
        
            io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 检查是否要退出
        if guess.trim().eq("quit") {
            println!("Thanks for playing!");
            break;
        } 

        println!("you guessed:{}...{}",guess,secret_number);

        //let guess_u32:u32 = guess.trim().parse().expect("Please type a number!");
        let guess_u32:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => break,//continue,
        };
        match guess_u32.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    
       
    
        
    }

}
