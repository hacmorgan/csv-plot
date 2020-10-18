mod args;
mod stdin;
mod plot;


pub struct Dataset
{
    columns           : [ i8 ; 3 ],
    _accumulator_size : u16,
    points            : Vec < [ f32 ; 3 ] >,
    colour            : &'static str,
}


fn main()
{

    let d : Dataset = args::initialise();
    println!( "Inferred columns: {:?}", d.columns );

    stdin::accumulate( d );
    
}
