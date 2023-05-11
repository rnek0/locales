#include <stdio.h>
#include <stdlib.h>
#include "langLocale.h"

int  main ( int argc, char *argv[], char *envp[] ) 
{ 
  printf("%s\n",listEnv());
  return  EXIT_SUCCESS ; 
}