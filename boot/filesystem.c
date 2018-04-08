#include <bmfs/bmfs.h>

int my_disk_read(void *disk_data, void *buf, bmfs_uint64 len, bmfs_uint64 *read_len);
   
int my_disk_seek(void *disk_data, bmfs_uint64_t pos, int whence);
   
void open_file(const char *path) {
   
    struct BMFSDisk disk;
   
    bmfs_disk_init(&disk);
    disk.seek = my_disk_seek;
    disk.read = my_disk_read;
}
   
/* Implement my_disk_read and my_disk_seek */

int open_file(const char *path) {
   
    /* Initialize disk here */
   
    struct BMFS bmfs;
   
    bmfs_init(&bmfs);
   
    bmfs_set_disk(&bmfs, &disk);
   
    int err = bmfs_import(&bmfs);
    if (err != 0) {
        kprintf("Failed to import BMFS file system.\n");
        return -1;
    }
}

int show_file(const char *path) {
   
    /* Initialize disk here. */
   
    /* Initialize file system header. */
   
    struct BMFSFile file;
   
    bmfs_file_init(&file);
   
    err = bmfs_open_file(&bmfs, &file, path);
    if (err == BMFS_ENOENT) {
        kprintf("Entry '%s' does not exist.\n", path);
    } else if (err == BMFS_EISDIR) {
        kprintf("Entry '%s' is a directory.\n", path);
    } else if (err != 0) {
        kprintf("Failed to open '%s'.\n", path);
        return -1;
    }
   
    bmfs_file_set_mode(&file, BMFS_MODE_READ);
   
    char buf[512];
   
    while (bmfs_file_eof(&file)) {
   
        bmfs_uint64 read_count = 0;
   
        err = bmfs_file_read(&file, buf, 512, &read_count);
        if (err != 0)
            break;
   
        my_print_function(buf, read_count);
    }
}
