use const_base::{Config, decode};

fn main(){
    decode!(b"+/", Config::B64);
    decode!(b"A===", Config::B64.end_padding(true));
    decode!(b"AAAAA", Config::B64);
    decode!(b"AAA\x00AA", Config::B64);
}