#include "asm-generic/errno-base.h"
#include "linux/device.h"
#include "linux/device/class.h"
#include "linux/init.h"
#include "linux/kern_levels.h"
#include "linux/printk.h"
#include "linux/stddef.h"
#include "linux/types.h"
#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/ioctl.h>
#include <linux/module.h>
#include <linux/uaccess.h>

#define MY_IOC_MAGIC 'M' // 매직 넘버
#define MY_IOCTL_WRITE                                                         \
    _IOW(MY_IOC_MAGIC, 0, char *) // 쓰기 작업을 위한 ioctl 명령
#define MY_IOCTL_READ                                                          \
    _IOR(MY_IOC_MAGIC, 1, char *) // 읽기 작업을 위한 ioctl 명령

static char kernel_buffer[256] = {0}; // 커널 버퍼
static dev_t dev;                     // 디바이스 번호
static struct cdev my_cdev;           // 캐릭터 디바이스 구조체
static struct class *cl;              // 디바이스 클래스

static long device_ioctl(struct file *file, unsigned int cmd,
                         unsigned long arg) {
    switch (cmd) {
    case MY_IOCTL_WRITE:
        // 사용자 공간에서 커널 공간으로 데이터 복사
        if (copy_from_user(kernel_buffer, (char *)arg, sizeof(kernel_buffer))) {
            return -EFAULT;
        }
        printk(KERN_INFO "Received from user: %s\n",
               kernel_buffer); // 로그 출력
        break;
    case MY_IOCTL_READ:
        // 커널 공간에서 사용자 공간으로 데이터 복사
        if (copy_to_user((char *)arg, kernel_buffer, sizeof(kernel_buffer))) {
            return -EFAULT;
        }
        break;
    default:
        return -EINVAL;
    }

    return 0;
}

// 파일 작업 구조체
static struct file_operations fops = {
    .unlocked_ioctl = device_ioctl, // ioctl 핸들러 지정
    .owner = THIS_MODULE,
};

static int __init ioctl_init(void) {
    // 캐릭터 디바이스 번호 할당
    if (alloc_chrdev_region(&dev, 0, 1, "my_ioctl_device") < 0) {
        return -1;
    }
    // 디바이스 클래스 생성
    if ((cl = class_create("chardev")) == NULL) {
        unregister_chrdev_region(dev, 1);
        return -1;
    }
    // 디바이스 생성
    if (device_create(cl, NULL, dev, NULL, "my_ioctl_device") == NULL) {
        class_destroy(cl);
        unregister_chrdev_region(dev, 1);
        return -1;
    }
    // 캐릭터 디바이스 초기화
    cdev_init(&my_cdev, &fops);
    // 캐릭터 디바이스 추가
    if (cdev_add(&my_cdev, dev, 1) == -1) {
        device_destroy(cl, dev);
        class_destroy(cl);
        unregister_chrdev_region(dev, 1);
        return -1;
    }

    return 0;
}

// 모듈 종료 함수
static void __exit ioctl_exit(void) {
    // 캐릭터 디바이스 삭제
    cdev_del(&my_cdev);
    // 디바이스 및 클래스 제거
    device_destroy(cl, dev);
    class_destroy(cl);
    // 디바이스 번호 해제
    unregister_chrdev_region(dev, 1);
}

module_init(ioctl_init);
module_exit(ioctl_exit);
MODULE_LICENSE("GPL");
MODULE_AUTHOR("RustWithLinux");
MODULE_DESCRIPTION("A simple ioctl example");
