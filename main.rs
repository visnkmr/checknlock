use window_titles::{Connection, ConnectionTrait};

use std::process::Command;
use std::process::{self, ExitStatus};
use std::io::{Read, Write};
use std::{
    env,
    process::{Stdio},
};
use execute::{Execute, shell};
use std::thread;
use schedule_recv::periodic_ms;
fn runpurerustshell(m: i32) -> i32{
    let mut k=m;
    let mut command = shell("xprop -id $(xprop -root _NET_ACTIVE_WINDOW | cut -d ' ' -f 5) WM_NAME | awk -F '\"' '{print $2}'");
    let mut command1 = shell("pacmd list-sink-inputs | grep -c 'state: RUNNING'");

command.stdout(Stdio::piped());
command1.stdout(Stdio::piped());

let output = command.execute_output().unwrap();
let output1 = command1.execute_output().unwrap();

let stdout1 = String::from_utf8(output.stdout).unwrap();
let stdout2 = String::from_utf8(output1.stdout).unwrap();

if(stdout1.to_lowercase().contains("twitch")||stdout1.to_lowercase().contains("youtube")){
    if(stdout2.contains("1")){
        println!("active window vid playing");
        k = 0;
        return k;

    }
    else{
        println!("not playing but open for {}",k);
        k+=1;
        if(k>120){
            k=0;
            let mut res = Command::new("xflock4").output();
        }
        return k;
    }
}
checkifplaying();
    k=0;
    return k;
}
fn runitnotrust(m: i32) -> i32{
    let mut k=m;
    // let connection = Connection::new().unwrap();
    let mut res = Command::new("./activewinname.sh")
    
    // .arg("-c")
    // .arg("pacmd")
    // .arg("list-sink-inputs")
    .stdout(Stdio::piped())
    // .expect("failed to execute process")
    .output()
    .unwrap()
    ;
    let stdout1 = String::from_utf8(res.stdout).unwrap();
    println!("{}",stdout1);
    if(stdout1.to_lowercase().contains("twitch")||stdout1.to_lowercase().contains("youtube"))
    {
    // for i in connection.window_titles().unwrap(){
        // if(i.to_lowercase().contains("twitch")||i.to_lowercase().contains("youtube")){
            
            let mut res1 = Command::new("./cal.sh")
    
    // .arg("-c")
    // .arg("pacmd")
    // .arg("list-sink-inputs")
    .stdout(Stdio::piped())
    // .expect("failed to execute process")
    .output()
    .unwrap()
    ;
    let stdout = String::from_utf8(res1.stdout).unwrap();
    // let mut res1 =res.unwrap().stdout;
    // let k = String::from_utf8(res1).unwrap();
    
    if(stdout.contains("1")){
        println!("active window vid playing");
        k = 0;
        return k;

    }
    else{
        println!("not playing but open for {}",k);
        k+=1;
        if(k>120){
            k=0;
            let mut res = Command::new("xflock4").output();
        }
        return k;
    }
        // }
        // else{
        // }
        // }
    }
    checkifplaying();
    k=0;
    return k;
}
fn checkifplaying(){
    let mut res1 = Command::new("./cal.sh")
    
    // .arg("-c")
    // .arg("pacmd")
    // .arg("list-sink-inputs")
    .stdout(Stdio::piped())
    // .expect("failed to execute process")
    .output()
    .unwrap()
    ;
    let stdout = String::from_utf8(res1.stdout).unwrap();
    // let mut res1 =res.unwrap().stdout;
    // let k = String::from_utf8(res1).unwrap();
    
    if(stdout.contains("1")){
        println!("vid playing background");
        // k = 0;
        // return k;

    }
    else{
        println!("not playing");
        // k+=1;
        // if(k>120){
        //     k=0;
        //     let mut res = Command::new("xflock4").output();
        // }
        // return k;
    }
}
fn runit(m: i32) -> i32{
    let mut k=m;
    let connection = Connection::new().unwrap();
    
    for i in connection.window_titles().unwrap(){
        if(i.to_lowercase().contains("twitch")||i.to_lowercase().contains("youtube")){
            
            let mut res = Command::new("./cal.sh")
    
    // .arg("-c")
    // .arg("pacmd")
    // .arg("list-sink-inputs")
    .stdout(Stdio::piped())
    // .expect("failed to execute process")
    .output()
    .unwrap()
    ;
    let stdout = String::from_utf8(res.stdout).unwrap();
    // let mut res1 =res.unwrap().stdout;
    // let k = String::from_utf8(res1).unwrap();
    
    if(stdout.contains("1")){
        println!("site open vid playing");
        k = 0;
        return k;

    }
    else{
        println!("not playing but open for {}",k);
        k+=1;
        if(k>120){
            k=0;
            let mut res = Command::new("xflock4").output();
        }
        return k;
    }
        }
        // else{
        
        // }
    }
    checkifplaying();
    k=0;
    return k;
}
fn main() {
    // runpurerustshell();
    let mut k=0;
    let tick = periodic_ms(2000);
    thread::sleep_ms(1000);
    let tock = periodic_ms(2000);
    loop {
        tick.recv().unwrap();
        // println!("Tick");
        k = runpurerustshell(k);
        tock.recv().unwrap();
        k = runpurerustshell(k);
        // println!("Tock");
    }
}
