use gnuplot;
use crate::Dataset;
use std::convert::TryInto;


pub fn new_figure() -> gnuplot::Axes2D
{
    gnuplot::Figure::new().axes2d()
}

pub fn update_plot( ax : &gnuplot::Axes2d , data : Dataset )
{
    xs = to_vector();
    ax.points();
        
    for i in 0..data.accumulator_size {
        let index : usize = i.try_into().unwrap();
        println!( "x: {}, y: {}, z: {}",
                   data.points[index][0],
                   data.points[index][1],
                   data.points[index][2] );
    }
}
