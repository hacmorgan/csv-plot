use gnuplot;
use crate::Dataset;


// pub fn new_figure() -> &'static mut gnuplot::Figure
// {
//     // gnuplot::Figure::new().axes2d()
//     gnuplot::Figure::new()
// }

// pub fn update_plot( ax : &gnuplot::Figure , data : Dataset )
pub fn update_plot( data : Vec < Dataset > )
{
    let mut fg = gnuplot::Figure::new();
    let ax = fg.axes2d();

    for d in data {
        let xs = to_vector( &d.points, 0 );
        let ys = to_vector( &d.points, 1 );
        ax.points( &xs, &ys, &[gnuplot::Color(d.colour)] );
    }
    
    match fg.show() {
        Ok(_) => std::process::exit(0),
        Err(err) => eprintln!( "error: {:?}", err ),
    }
}

fn to_vector( points : &Vec < [ f32 ; 3 ] > , index : usize ) -> Vec < f32 >
{
    let mut ret : Vec < f32 > = Vec::new();
    for row in points {
        ret.push( row[index] );
    }
    ret
}
