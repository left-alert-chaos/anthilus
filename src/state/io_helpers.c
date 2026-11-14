#include <termios.h>
#include <unistd.h>
#include <fcntl.h>

struct termios original_config;

void c_enter_raw_mode()
{
	tcgetattr(0, &original_config);
	struct termios new_config = original_config;
	new_config.c_lflag &= ~(ECHO | ICANON);
	tcsetattr(0, TCSAFLUSH, &new_config);
}

void c_exit_raw_mode()
{
	tcsetattr(0, TCSAFLUSH, &original_config);
}

// Basically stolen from this URL:
// https://csresources.github.io/SystemProgrammingWiki/SystemProgramming/Networking,-Part-7:-Nonblocking-I-O,-select(),-and-epoll/
void c_set_stdin_nonblocking()
{
	int stdin_flags = fcntl(0, F_GETFL, 0);
	fcntl(0, F_SETFL, stdin_flags | O_NONBLOCK);
}
