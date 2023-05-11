#include <stdlib.h>

extern char **environ;

const char *listEnv()
{
  char **env = environ;
  if (getenv("LANGUAGE") != NULL){
    return getenv("LANGUAGE");
  }
  else {
    return "en";
  }
}
