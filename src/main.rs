mod args;
mod stdin;
mod plot;


/**

csv-plot: plot data on stdin using gnuplot

 */


/*
TODO:
- allow setting plot type and colour, marker etc
  - work out how to convert 
    &[gnuplot::PlotOption<std::string::String>] -> &[gnuplot::PlotOption<&str>]
- add verbose flag 
  - make sure it's possible to also check for the help flag
  - display more detailed info on the formatiing of --fields and --format
  - show examples
- add usage examples
- allow multiple plots to be spawned
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
    let mut datasets : Vec < Dataset > = args::initialise();

    datasets = stdin::accumulate( datasets );

    plot::plot( datasets );
}
