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
