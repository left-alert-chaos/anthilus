#include <sys/ioctl.h>

int terminal_width()
{
	struct winsize size;
	ioctl(0, TIOCGWINSZ, &size);
	return size.ws_col;
}
