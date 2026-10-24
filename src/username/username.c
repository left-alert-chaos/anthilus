#include <unistd.h>
#include <stdlib.h>

char *c_user()
{
	// fuck you if your username is more than 100 characters
	char *uname = (char *)malloc(101);
	getlogin_r(uname, 100);
	return uname;
}
