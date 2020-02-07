#include <stdio.h>      // Enlever lors de l'appel de rust
#include <stdlib.h>

extern char **environ;

char *listEnv ()
{
  char **env = environ;
  return getenv("LANGUAGE");
}

int  main ( int argc, char *argv[], char *envp[] ) 
{ 
  // Enlever lors de l'appel de rust
  printf("%s\n",listEnv());

  return  EXIT_SUCCESS ; 
}