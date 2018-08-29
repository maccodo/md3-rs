extern crate md3_rs;
use md3_rs::md3;

fn main ()
{
    use std::env;
    let argv : Vec<String> = env::args().collect();
    let fname = argv[1].clone();
    println!("FILENAME: {}", fname);
    let _md3_model = md3::Md3Model::load( fname ).unwrap();

    println!("LOCAL_ORGIGIN[0].x: {}", _md3_model.frames[0].radius );
}
