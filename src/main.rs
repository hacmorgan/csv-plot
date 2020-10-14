use gnuplot::{ Figure, Caption, Color };
use std::io::{ self, Read };
use std::mem;  // for Box


/*

TODO:
// start with only one dataset at a time (add functionality in pieces)
- test that data is being correctly read into Dataset object
- complete plot() 

 */


struct Dataset
{
    x       : &[u32],
    y       : &[u32],
    color   : &[u8],  // &[u8] == &str
}


fn read_data() -> Box <Dataset>
{
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_to_string( &mut buffer );

    Box::new( Dataset() )
}


fn plot( d : Dataset )
{
    ;
    // let x : [u32; 3] = [ 0, 1, 2 ];
    // let y : [u32; 3] = [ 3, 4, 5 ];

    // let mut fg = Figure::new();

    // fg.axes2d()
    //     .lines(  &x,
    //              &y,
    //              &[Caption("A line"), Color("black")] );
    // fg.show().unwrap();
}


fn main()
{

    dset : Box <Dataset> = read_data();
    plot( dset );
    
}
