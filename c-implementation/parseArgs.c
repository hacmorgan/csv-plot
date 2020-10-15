#include <stdio.h>

#include "csv-plot.h"


/**  returns argv at the point where *argv == key, NULL if not present  */
char **contains( char *key, char **argv )
{
    if ( argv == NULL ) { 
        return NULL;
    }
    if ( strcmp(key, *argv) == 0 ) {
        return argv;
    } else {
        return contains( key, argv++ );
    }
}


void checkForHelp( char **argv )
{
    if ( contains("-h", argv) != NULL || contains("--help", argv) != NULL ) {
        help();
        exit(0);
    }
}


char *getFields( char **argv )
{
    if ( contains("-f", argv) == NULL && contains("--fields", argv) == NULL ) {
        return NULL;
        fprintf( stderr, "error: --fields required\n" )
    } else if ( *(argv++) == NULL ) {
        return NULL;
        fprintf( stderr, "error: --fields has no definition\n" )
    } else {
        return *argv;
    }
}


Data *parseArgs( char **argv )
{
    checkForHelp();
    char *fields = getFields( argv );
    /* recursively check for x, x1, x2 etc (new function) */
}
