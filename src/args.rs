extern crate clap;

use std::convert::TryInto;
use crate::Dataset;


pub fn initialise() -> Vec < Dataset >
{
    fn check_verbose( args : clap::ArgMatches< 'static > )
    {
        if let Some( arg ) = args.value_of("verbose") {
            eprintln!("verbose not yet implemented");
        }
    }
    
    let given_args = get_args();
    let fields = given_args.value_of("fields").unwrap();
    let format = given_args.value_of("format").unwrap();
    let mut datasets : Vec < Dataset > = Vec::new();
    
    for x in find_xs( fields ) {
        datasets.push(
            Dataset {
                columns           : infer_columns( x, fields ),
                _accumulator_size : 10,
                points            : Vec::new(),
                format            : find_format( x, format ),
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


fn find_format( x : &str, format : &str ) -> &'static [gnuplot::PlotOption<&'static str>]
{
    /** rectify dataset names and construct Vec < (name, format) > */
    fn parse_format( format : &str ) -> Vec < ( &str, &str ) >
    {
        fn iter_to_str( iter : Vec < &str > ) -> &str
        {
            let flat = String::new();
            for s in iter {
                flat += s;
            }
            &flat
        }
        
        let mut format_vector : Vec < (&str, &str) > = Vec::new();

        for substring in format.split(";") {
            let fields = substring.split(",");
            if let Some(x) = fields.next() {
                format_vector.push( (rectify_x(x), iter_to_str(fields.collect())) )
            } else {
                eprintln!("Received a bad input: {}", substring);
            }
        }
        
        format_vector
    }

    /** x -> xa0, x1 -> xa1, xb -> xb0 */
    fn rectify_x( x : &str ) -> &str
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
                let c = c2.unwrap();
                if c.is_alphabetic() {
                    '0'
                } else {
                    c
                }
            } else {
                c2.unwrap()
            }
        }

        let chars = x.chars();
        let mut rectified = String::new();
        
        rectified.push( 'x'            );  // first is guaranteed to be x
        rectified.push( get_figure(x)  );  // dataset's letter
        rectified.push( get_dataset(x) );  // dataset's number

        rectified.as_str()
    }

    fn plot_options( format : &str ) -> Vec < gnuplot::PlotOption<&'static str> >
    {
        let mut plot_opts : Vec < gnuplot::PlotOption<&'static str> > = Vec::new();
        for opt in format.split(",") {
            match 
        }
    }

    let format_vector : Vec < (&str, &str) > = parse_format( format );
    for dataset_format in format_vector {
        match dataset_format {
            (x, f) => plot_options(f),
            _      => continue,
        }
    }
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
        .version( "0.0" )
        .author( "Hamish Morgan" )
        .about( "plot data from stdin using gnuplot" )
        .arg( clap::Arg::with_name("fields")
              .short("f")
              .long("fields")
              .help("format of data on stdin")
              .takes_value(true)
              .default_value("x,y") )
        .arg( clap::Arg::with_name("format")
              .short("m")
              .long("format")
              .help("plot style for each dataset")
              .takes_value(true) )
        .arg( clap::Arg::with_name("verbose")
              .short("v")
              .long("verbose")
              .help("more output") )
        .get_matches()
}
