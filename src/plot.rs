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
    fn set_highest( highest : u8, new : u8 ) -> u8
    {
        if new > highest {
            new
        } else {
            highest
        }
    }
    
    let mut highest : u8 = 0;
    
    for d in datasets {
        match d.columns {
            [ -1, -1, -1 ] => continue,
            [  _, -1, -1 ] => highest = set_highest( highest, 1 ),
            [  _,  _, -1 ] => highest = set_highest( highest, 2 ),
            [  _,  _,  _ ] => highest = set_highest( highest, 3 ),
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
        let colour_string  = get_colour( &d.format );
        let caption_string = get_caption( &d.format );
        let colour  = gnuplot::Color( &*colour_string );
        let caption = gnuplot::Caption( &*caption_string );
        match &*d.style {
            "points" => ax.points( &xs, &ys, &[colour, caption] ),
            "lines"  => ax.lines(  &xs, &ys, &[colour, caption] ),
            _        => {
                eprintln!( "error: --plot-style accepts only points or lines" );
                std::process::exit(1);
            },
        };
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
        let colour_string = get_colour(&d.format);
        let colour = gnuplot::Color( &*colour_string );
        let caption_string = get_caption(&d.format);
        let caption = gnuplot::Caption( &*caption_string );
        match &*d.style {
            "points" => ax.points( &xs, &ys, &zs, &[colour, caption] ),
            "lines"  => ax.lines(  &xs, &ys, &zs, &[colour, caption] ),
            _        => {
                eprintln!( "error: --plot-style accepts only points or lines" );
                std::process::exit(1);
            },
        };
    }
}


fn get_colour( fmt : &Option< Vec< (String, String) > > ) -> String
{
    if let Some(fmt_vec) = fmt {
        for (name, value) in fmt_vec {
            if name == "colour" {
                return value.to_string()
            }
        }
    }
    String::from("red")
}


fn get_caption( fmt : &Option< Vec< (String, String) > > ) -> String
{
    if let Some(fmt_vec) = fmt {
        for (name, value) in fmt_vec {
            if name == "caption" {
                return value.to_string()
            }
        }
    }
    String::from("plot")
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
                _         => eprintln!( "unknown format argument: {}", name ),
            }
        }
    }

    gnuplot_vec
}
