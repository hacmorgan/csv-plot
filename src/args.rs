extern crate clap;

use std::convert::TryInto;
use crate::Dataset;


pub fn initialise() -> Vec< Dataset >
{
    fn check_verbose( args : &clap::ArgMatches<'static> )
    {
        if let Some( arg ) = args.value_of("verbose") {
            eprintln!("verbose not yet implemented");
        }
    }

    fn check_explain( args : &clap::ArgMatches<'static> )
    {
        if args.is_present( "explain" ) {
            explain();
            std::process::exit(0);
        }
    }
    
    let given_args = get_args();
    check_explain( &given_args );
    let fields = given_args.value_of("fields").unwrap();
    let style = match given_args.value_of("style") {
        Some(f) => f.to_owned(),
        None    => String::from(""),
    };
    let mut datasets : Vec< Dataset > = Vec::new();
    
    for x in find_xs( fields ) {
        datasets.push(
            Dataset {
                columns           : infer_columns( x, fields ) ,
                _accumulator_size : 10                         ,
                points            : Vec::new()                 ,
                style             : find_style( x, &style )  ,
            }
        )
    }

    datasets
}


fn replace_first_char( input : &str, replacement : char ) -> String
{
    input
        .chars()
        .enumerate()
        .map( |(i,c)| if i == 0 { replacement } else { c } )
        .collect()
}


/** determine how many datasets there are by counting the number of x_
    fields given, and verify that a matching y_ field was given */
fn find_xs( fields : &str ) -> Vec < &str >
{
    fn is_valid_field( field : &str, separated : &mut std::str::Split <&str> ) -> bool
    {
        if field.chars().next() != Some('x') {
            return false
        }
        let y_equiv : String = replace_first_char( &field, 'y' );        
        if separated.position( |f| f == &y_equiv ) != None {
            true
        } else {
            false
        }
    }
    
    let mut ret : Vec < &str > = Vec::new();
    let mut separated = fields.trim().split(",");
    
    for field in separated.clone() {
        if is_valid_field( &field, &mut separated ) {
            ret.push(field);
        }
    }

    ret
}


fn find_style( x : &str, style : &String )
                -> Option< Vec< (String, String) > >
{
    /** rectify dataset names and construct Vec < (name, style) > */
    fn parse_style( style : String ) -> Vec < (String, String) >
    {
        fn iter_to_str( iter : Vec < &str > ) -> String
        {
            let mut flat = String::new();
            for s in iter {
                flat += s;
                flat += ",";
            }
            flat
        }
        
        let mut style_vector : Vec < (String, String) > = Vec::new();

        for substring in style.split(";") {
            let mut fields = substring.split(",");
            if let Some(x) = fields.next() {
                style_vector.push( (rectify_x(x), iter_to_str(fields.collect())) )
            } else {
                eprintln!("Received a bad input: {}", substring);
            }
        }
        
        style_vector
    }

    /** x -> xa0, x1 -> xa1, xb -> xb0 */
    fn rectify_x( x : &str ) -> String
    {
        fn get_figure( x : &str ) -> char
        {
            let c = x.chars().nth(1);
            match c {
                None    => 'a',
                Some(c) => {
                    if c.is_alphabetic() {
                        c
                    } else {
                        'a'
                    }
                }
            }
        }

        fn get_dataset( x : &str ) -> char
        {
            let c1 = x.chars().nth(1);
            let c2 = x.chars().nth(2);
            if c1 == None && c2 == None {
                '0'
            } else if c2 == None {
                let c = c1.unwrap();
                if c.is_alphabetic() {
                    '0'
                } else {
                    c
                }
            } else {
                c2.unwrap()
            }
        }

        let mut rectified = String::new();
        rectified.push( 'x'            );  // first is guaranteed to be x
        rectified.push( get_figure(x)  );  // dataset's letter
        rectified.push( get_dataset(x) );  // dataset's number

        rectified
    }

    fn plot_options( dataset_style : String ) -> Vec< (String, String) >
    {
        let mut plot_opts : Vec< (String, String) > = Vec::new();
        for opt in dataset_style.split(",") {
            let mut name_value = opt.split("=");
            if let (Some(name), Some(value)) = (name_value.next(), name_value.next()) {
                plot_opts.push( (String::from(name), String::from(value)) );
            }
        }
        plot_opts
    }

    let style_vector : Vec < (String, String) > = parse_style( style.to_string() );
    for (name, value) in style_vector {
        if name == rectify_x(x) {
            return Some( plot_options(value) );
        }
    }

    None
}


/** return column numbers for fields matching the pattern of x */
fn infer_columns( x : &str , fields : &str ) -> [ i8 ; 3 ]
{
    fn extract( val : usize ) -> i8
    {
        val.try_into().unwrap()
    }
    
    let mut columns : [ i8 ; 3 ] = [ -1, -1, -1 ];
    let separated = fields.split(",");

    for (count, field) in separated.enumerate() {
        if field == x {
            columns[0] = extract(count);
        } else if field == replace_first_char( x, 'y' ) {
            columns[1] = extract(count);
        } else if field == replace_first_char( x, 'z' ) {
            columns[2] = extract(count);
        }
    }

    columns
}


fn get_args() -> clap::ArgMatches< 'static > 
{
    clap::App::new( "csv-plot" )
        .version( "0.1" )
        .author( "Hamish Morgan" )
        .about( "\nplot data from stdin using gnuplot" )
        .arg( clap::Arg::with_name("fields")
              .short("f")
              .long("fields")
              .help("style of data on stdin")
              .takes_value(true)
              .default_value("x,y") )
        .arg( clap::Arg::with_name("style")
              .short("s")
              .long("style")
              .help("per-dataset style (e.g. colour, caption)")
              .takes_value(true) )
        .arg( clap::Arg::with_name("verbose")
              .short("v")
              .long("verbose")
              .help("more output") )
        .arg( clap::Arg::with_name("explain")
              .short("e")
              .long("explain")
              .help("print usage examples and more detailed explanations") )
        .get_matches()
}


fn explain()
{
    eprintln!( "SPECIFYING INPUT COLUMNS (--fields)"                                );
    eprintln!( "csv-plot can plot in 2D or 3D, and will automatically choose based" );
    eprintln!( "on the specified input columns."                                    );
    eprintln!(                                                                      );
    eprintln!( "SPECIFYING THE STYLE OF A DATASET (--style)"                       );
    eprintln!( "A dataset's x label is used to specify its style settings, as  "   );
    eprintln!( "<xlabel>,<option>,<option>;<xlabel>,<option>"                       );
    eprintln!(                                                                      );
    eprintln!( "EXAMPLES"                                                           );
    eprintln!( "plot two sets of 2D data in red and blue"                           );
    eprintln!( "    cat data.csv | csv-plot --fields \"x0,y0,x1,y1\" \\"            );
    eprintln!( "        --style \"x0,colour=red;x1,colour=blue\""                  );
    eprintln!( "plot three 3D datasets, give each a caption and colour one of them" );
    eprintln!( "    cat data.csv | csv-plot --fields \"x,x1,x2,y,y1,y2,z,z1,z2\"\\" );
    eprintln!( "        --style \"x,caption=good;x1,caption=bad;x2,caption=okay\"" );
}
