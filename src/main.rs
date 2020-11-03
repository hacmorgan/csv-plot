mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
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
    let (mut datasets, verbose) = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets, verbose );
}
