mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
- allow multiple plots to be spawned
- change format to style
- allow per-plot legend, xlabel, ylabel
- add verbose flag 
  - print info on columns (this currently happens every time)
  - if it's possible to check for help:
    - display more detailed info on the formatting of --fields and --format
    - show examples
- add usage examples
- code cleanup
  - investigate potentially moving gnuplot::PlotOption objects back out of plot.rs
    - to_owned() may be helpful
  - remove the Option for format, insert defaults at arg-parsing time
 */


pub struct Dataset
{
    columns           : [ i8 ; 3 ]                        ,
    _accumulator_size : u16                               ,
    points            : Vec< [ f32 ; 3 ] >                ,
    style             : Option< Vec< (String, String) > > ,
}


fn main()
{
    let mut datasets : Vec<Dataset> = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets );
}
