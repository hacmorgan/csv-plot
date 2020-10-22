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


fn find_format( x : &str, format : &str ) -> &[gnuplot::PlotOption<&str>]
{
    fn parse_format( format : &str ) -> Vec < ( &str, &str ) >
    {
        ;
    }

    fn rectify_x( x : &str ) -> &str
    {
        let mut rectified = Vec::new();
        let chars = x.chars();

        rectified.push( chars.next() );  // first is guaranteed to be x

        let next = chars.next();
        if next.is_alphabetic() {
            rectified.push( next );
        } else {
            rectified.push( 'a' );
        }

        let next = chars.next();
        match next {
            Some(n) => rectified.push( n ),
            None    => rectified.push( 0 ),
        }

        rectified.as_str()
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
