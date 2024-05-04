/*
    Enums allows us to enumarate a list of variants
*/

enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn some_function(){}
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}


fn main(){
    // Examples from when IpAddrKind didn't store values
    // let four = IpAddKind::V4;
    // let six = IpAddKind::V6;
    let localhost = IpAddrKind::V4(127,0,0,1);

}

fn route(ip_kind: IpAddrKind){}
