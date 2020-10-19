mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */
/*
TODO:
- enable multiple datasets to be plotted
  - data.points.push line in stdin.rs should become a function that handles 
    multiple datasets
- enable 3D plotting (update_plot() should handle this)
- allow setting plot type and colour, marker etc
- add usage examples
 */


pub struct Dataset
{
    columns           : [ i8 ; 3 ],
    _accumulator_size : u16,
    points            : Vec < [ f32 ; 3 ] >,
    colour            : &'static str,
}


fn main()
{

    let d : Vec < Dataset > = args::initialise();
    // println!( "Inferred columns: {:?}", d.columns );

    stdin::accumulate( d );
    
}
