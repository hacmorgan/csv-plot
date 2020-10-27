mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
- allow multiple plots to be spawned
- add --plot-style to allow lines instead of points
- add verbose flag 
  - print info on columns (this currently happens every time)
  - if it's possible to check for help:
    - display more detailed info on the formatting of --fields and --format
    - show examples
- add usage examples
- investigate potentially moving gnuplot::PlotOption objects back out of plot.rs
 */


pub struct Dataset
{
    columns           : [ i8 ; 3 ],
    _accumulator_size : u16,  // not sure if accumulating input is feasible
    points            : Vec< [ f32 ; 3 ] >,
    format            : Option< Vec< (String, String) > >,
}


fn main()
{
    let mut datasets : Vec<Dataset> = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets );
}
