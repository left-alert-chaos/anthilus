import os

os.mkdir("files_to_read")

for i in range(100):
    open(f"files_to_read/file{i}", "w").close()
