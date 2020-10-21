mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
- allow setting plot type and colour, marker etc
- add verbose flag (read clap docs to see if this is already implemented)
- add usage examples
- allow multiple plots to be spawned
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
    let mut datasets : Vec < Dataset > = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets );
}
