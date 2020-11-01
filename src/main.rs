mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
- accumulate data
- add verbose flag 
  - print info on columns (this currently happens every time)
  - if it's possible to check for help:
    - display more detailed info on the formatting of --fields and --format
    - show examples
- add usage examples
- add binary data support
 */


#[derive(Debug)]
pub struct Dataset
{
    columns          : [ i8 ; 3 ]                        ,
    accumulator_size : u16                               ,
    points           : Vec< [ f32 ; 3 ] >                ,
    plot             : char                              ,
    style            : Option< Vec< (String, String) > > ,
    plot_options     : Option< Vec< (String, String) > > ,
}


fn main()
{
    let mut datasets : Vec<Dataset> = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets );
}
