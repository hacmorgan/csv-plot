use gnuplot;
use crate::Dataset;


pub fn plot( data : Vec < Dataset > )
{
    let mut fg = gnuplot::Figure::new();

    let highest : u8 = highest_dimension( &data );
    if highest == 2 {
        plot2d( &mut fg, data );
    } else if highest == 3 {
        plot3d( &mut fg, data );
    }
    
    match fg.show() {
        Ok(_) => std::process::exit(0),
        Err(err) => eprintln!( "error: {:?}", err ),
    }
}


fn highest_dimension( datasets : &Vec < Dataset > ) -> u8
{
    let mut highest : u8 = 0;
    
    for d in datasets {
        match d.columns {
            [ -1, -1, -1 ] => continue,
            [  _, -1, -1 ] => highest = 1,
            [  _,  _, -1 ] => highest = 2,
            [  _,  _,  _ ] => highest = 3,
        }
    }

    highest
}


fn plot2d( fg : &mut gnuplot::Figure , data : Vec < Dataset > )
{
    let ax = fg.axes2d();

    for d in data {
        eprintln!( "Got dataset with columns: {:?}", d.columns );
        let xs = to_vector( &d.points, 0 );
        let ys = to_vector( &d.points, 1 );
        ax.points( &xs, &ys, &gnuplot_options(d.format) );
    }
}


fn plot3d( fg : &mut gnuplot::Figure , data : Vec < Dataset > )
{
    let ax = fg.axes3d();

    for d in data {
        eprintln!( "Got dataset with columns: {:?}", d.columns );
        let xs = to_vector( &d.points, 0 );
        let ys = to_vector( &d.points, 1 );
        let zs = to_vector( &d.points, 2 );
        ax.points( &xs, &ys, &zs, &gnuplot_options(d.format) );
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


fn gnuplot_options( format : Option< Vec< (String, String) > > )
                    -> Vec< gnuplot::PlotOption<String> >
{
    let mut gnuplot_vec : Vec< gnuplot::PlotOption<String> > = Vec::new();

    if let Some(f_vector) = format {
        for (name, value) in f_vector {
            let name_str : &str = &name;
            match name_str {
                "colour"  => gnuplot_vec.push( gnuplot::Color(value) ),
                "caption" => gnuplot_vec.push( gnuplot::Caption(value) ),
                other     => eprintln!( "unknown format argument: {}", name ),
            }
        }
    }

    gnuplot_vec
}
