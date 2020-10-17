// use gnuplot::{ Figure, Caption, Color };
// use std::io::{ self, Read };
// use std::mem;  // for Box

mod args;
// mod stdin;
// mod plot;


/*

TODO:
// start with only one dataset at a time (add functionality in pieces)
- test that data is being correctly read into Dataset object
- complete plot() 

 */


pub struct Dataset
{
    columns : [ i8 ; 3 ],
    points  : Vec < &'static [f32] >,
    colour  : &'static [u8],  // &[u8] == &str
}


fn main()
{

    let _d : Dataset = args::initialise();
    println!( "Inferred columns: {:?}", _d.columns );
}
