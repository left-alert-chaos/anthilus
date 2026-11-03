#include <termios.h>
#include <unistd.h>

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
