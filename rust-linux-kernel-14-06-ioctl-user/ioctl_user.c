#include <fcntl.h>
#include <stdio.h>
#include <sys/ioctl.h>
#include <unistd.h>

#define MY_IOC_MAGIC 'M' // 매직 넘버 정의
#define MY_IOCTL_WRITE                                                         \
    _IOW(MY_IOC_MAGIC, 0, char *) // 쓰기 작업을 위한 ioctl 명령 정의
#define MY_IOCTL_READ                                                          \
    _IOR(MY_IOC_MAGIC, 1, char *) // 읽기 작업을 위한 ioctl 명령 정의

int main() {
    // "/dev/my_ioctl_device" 디바이스 파일을 읽기/쓰기 모드로 열기
    int fd = open("/dev/my_ioctl_device", O_RDWR);
    if (fd == -1) {
        perror("open"); // 파일 읽기 실패 시 에러 메시지 출력
        return -1;
    }

    // 사용자 버퍼에 데이터 저장
    char user_buffer[256] = "Hello from user!";
    // ioctl로 커널로 데이터 쓰기
    if (ioctl(fd, MY_IOCTL_WRITE, user_buffer) == -1) {
        perror("ioctl write"); // ioctl 쓰기 실패 시 에러 메시지 출력
        close(fd);             // 파일 닫기
        return 1;
    }

    // 커널에서 읽은 데이터를 저장할 버퍼 초기화
    char read_buffer[256] = {0};
    // ioctl로 커널에서 데이터 읽기
    if (ioctl(fd, MY_IOCTL_READ, read_buffer) == -1) {
        perror("ioctl read"); // ioctl 읽기 실패 시 에러 메시지 출력
        close(fd);
        return 1;
    }

    // 커널에서 받은 데이터 출력
    printf("Received from kernel: %s\n", read_buffer);

    close(fd); // 파일 닫기
    return 0;
}
