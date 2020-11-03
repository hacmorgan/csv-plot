use gnuplot;
use gnuplot::AxesCommon;
use crate::Dataset;
use std::thread;


/** plot: split datasets up by plot, and call plotting routine for each */
pub fn plot( data : Vec< Dataset >, verbose : bool )
{
    fn plot_dataset( dataset : Vec< Dataset >, verbose : bool )
    {
        let mut fg = gnuplot::Figure::new();

        let highest : u8 = highest_dimension( &dataset );
        if highest == 2 {
            plot2d( &mut fg, dataset, verbose );
        } else if highest == 3 {
            plot3d( &mut fg, dataset, verbose );
        }
        
        match fg.show() {
            Ok(_) => (),
            Err(err) => eprintln!( "error: {:?}", err ),
        }
    }

    fn separate_by_plot( mut data : Vec< Dataset > ) -> Vec< Vec< Dataset > >
    {
        let mut plots        : Vec< Vec< Dataset > > = Vec::new();
        let mut datasets     :      Vec< Dataset >   = Vec::new();
        let mut current_plot : char = ' ';

        data.sort_by_key( |a| a.plot );
        for d in data {
            if current_plot == ' ' {
                current_plot = d.plot;
            }
            if d.plot == current_plot {
                datasets.push( d );
            } else {
                plots.push( datasets );
                current_plot = d.plot;
                datasets = Vec::new();
                datasets.push( d );
            }
        }
        plots.push( datasets );

        plots
    }

    let mut children : Vec< std::thread::JoinHandle<()> > = Vec::new();
    for plt in separate_by_plot( data ) {
        children.push(
            thread::spawn( move || {
                plot_dataset( plt, verbose );
            } )
        );
    }
    for child in children {
        let _ = child.join();
    }
}


fn highest_dimension( datasets : &Vec< Dataset > ) -> u8
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


fn plot2d( fg : &mut gnuplot::Figure , data : Vec < Dataset >, verbose : bool )
{
    fn new_2d_axes( fg        : &mut gnuplot::Figure,
                    plot_opts : Option< Vec< (String, String) > > )
                    -> &mut gnuplot::Axes2D
    {
        fg.axes2d()
            .set_x_label( &extract_or(&plot_opts, "xlabel", ""), &[] )
            .set_y_label( &extract_or(&plot_opts, "ylabel", ""), &[] )
            .set_title(   &extract_or(&plot_opts, "title",  ""), &[] )
    }
    
    let ax = new_2d_axes( fg, get_plot_options( &data ) );

    for d in data {
        if verbose {
            eprintln!( "Got dataset with columns: {:?}", d.columns );
        }
        let xs = to_vector( &d.points, 0 );
        let ys = to_vector( &d.points, 1 );
        match &*get_plot_type( &d.style ) {
            "points" => ax.points(  &xs, &ys, 
                                    &[gnuplot::Color(&*get_colour(&d.style)),
                                      gnuplot::Caption(&*get_caption(&d.style))] ),
            "lines"  => ax.lines(   &xs, &ys,
                                    &[gnuplot::Color(&*get_colour(&d.style)),
                                      gnuplot::Caption(&*get_caption(&d.style))] ),
            _        => {
                eprintln!( "error: \"type\" accepts only points or lines" );
                std::process::exit(1);
            },
        };
    }
}


fn plot3d( fg : &mut gnuplot::Figure , data : Vec< Dataset >, verbose : bool )
{
    fn new_3d_axes( fg        : &mut gnuplot::Figure,
                    plot_opts : Option< Vec< (String, String) > > )
                    -> &mut gnuplot::Axes3D
    {
        fg.axes3d()
            .set_x_label( &extract_or(&plot_opts, "xlabel", ""), &[] )
            .set_y_label( &extract_or(&plot_opts, "ylabel", ""), &[] )
            .set_z_label( &extract_or(&plot_opts, "zlabel", ""), &[] )
            .set_title(   &extract_or(&plot_opts, "title",  ""), &[] )
    }
    
    let ax = new_3d_axes( fg, get_plot_options( &data ) );

    for d in data {
        if verbose {
            eprintln!( "Got dataset with columns: {:?}", d.columns );
        }
        let xs = to_vector( &d.points, 0 );
        let ys = to_vector( &d.points, 1 );
        let zs = to_vector( &d.points, 2 );
        match &*get_plot_type( &d.style ) {
            "points" => ax.points( &xs, &ys, &zs,
                                    &[gnuplot::Color(&*get_colour(&d.style)),
                                      gnuplot::Caption(&*get_caption(&d.style))] ),
            "lines"  => ax.lines( &xs, &ys, &zs,
                                    &[gnuplot::Color(&*get_colour(&d.style)),
                                      gnuplot::Caption(&*get_caption(&d.style))] ),
            _        => {
                eprintln!( "error: \"type\" accepts only points or lines" );
                std::process::exit(1);
            },
        };
    }
}


fn get_plot_options( data : &Vec< Dataset > ) -> Option< Vec< (String, String) > >
{
    match data.iter().next() {
        Some(d) => d.plot_options.clone(),
        None    => {
            eprintln!( "error: no datasets supplied" );
            std::process::exit(1);
        },
    }
}


fn get_colour( style : &Option< Vec< (String, String) > > ) -> String
{
    extract_or( style, "colour", "red" )
}


fn get_caption( style : &Option< Vec< (String, String) > > ) -> String
{
    extract_or( style, "caption", "" )
}


fn get_plot_type( style : &Option< Vec< (String, String) > > ) -> String
{
    extract_or( style, "type", "lines" )
}


fn extract_or( options     : &Option< Vec< (String, String) > >,
               field       : &str,
               alternative : &str ) -> String
{
    if let Some(options_vec) = options {
        for (name, value) in options_vec {
            if name == field {
                return value.to_string()
            }
        }
    }
    String::from(alternative)
}


fn to_vector( points : &Vec < [ f32 ; 3 ] > , index : usize ) -> Vec < f32 >
{
    let mut ret : Vec < f32 > = Vec::new();
    for row in points {
        ret.push( row[index] );
    }
    ret
}
