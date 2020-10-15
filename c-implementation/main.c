#include <stdio.h>
#include <stdlib.h>

#include "gnuplot_i.h"

#define SLEEP_LGTH  3
#define MAXLINE     1000


/**

   csv-plot
   --------

   feed data from stdin to gnuplot using Nicolas Devillard's ANSI C interface


   TODO
   ----
   - blindly take data as x,y -> plot
     - test what happens with big datasets to find a good accsize
   - allow setting --fields to rearrange x and y
   - allow setting --fields to set plot style
   - allow more than one plot, with separate plot styles

 */


typedef struct List
{
    void *this;
    void *next;
} List;

typedef struct Point
{
    double xs;
    double ys;
} Point; 

typedef struct Data
{
    int indices[3];
    Point *points;
} Data;


Point *accumulatePoint( char *fields, int accumulator_size )
{
    char input[MAXLINE];
    Point *points[];
    int acc;

    points = (Point*) malloc( accumulator_size * sizeof(Point) + 1 );
    
    for ( acc = 0 ; gets(input) != NULL && acc < accumulator_size ; acc++ ) {
        
    }

    free(points);
}


int main( int argc, char **argv )
{
    gnuplot_ctrl *h;
    h = gnuplot_init();

    int i;
    gnuplot_resetplot( h );
    gnuplot_cmd( h, "plot sin(%d)", i );
    sleep( 0 );

    /* for ( i=0 ; i<10 ; i++ ) { */
    /*     gnuplot_cmd( h, "plot sin(%d)", i ); */
    /*     sleep( SLEEP_LGTH ); */
    /* } */
    
    gnuplot_close(h);
    
    return 0;
    
}
