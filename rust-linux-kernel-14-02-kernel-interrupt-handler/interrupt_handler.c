#include <asm-generic/errno-base.h>
#include <asm/io.h>
#include <linux/input.h>
#include <linux/interrupt.h>
#include <linux/kernel.h>
#include <linux/module.h>

static struct input_dev *my_device; // 입력 디바이스 구조체 포인터

// 인터럽트 핸들러 함수
static irqreturn_t interrupt_handler(int irq, void *dev_id) {
    pr_info("MY IRQ HANDLER CALLED: irq=%d\n", irq);

    static unsigned char scancode; // 키보드 스캔코드 저장을 위한 변수
    unsigned char status;          // 키보드 상태를 저장하는 변수

    status = inb(0x64); // 키보드 상태 읽기

    if (status & 0x01) {
        scancode = inb(0x60); // 스캔코드 읽기
        pr_info("scancode=%x status=%x\n", scancode, status);
    }

    pr_info("Key pressed with scancode: %x\n", scancode); // 스캔코드 출력
    if (status & 0x01 && !(scancode & 0x80)) {
        pr_info("Key pressed with scancode: %x\n",
                scancode); // 키보드 상태와 스캔코드를 확인해 키 눌림을 출력
    }

    return IRQ_HANDLED; // 인터럽트 처리 완료 표
}

// 모듈 초기화 함수
static int __init my_init(void) {
    // 키보드 인터럽트에 대한 핸들러 함수를 등록
    if (request_irq(1, interrupt_handler, IRQF_SHARED, "my_keyboard_interrupt",
                    &my_device)) {
        pr_err(
            "Failed to register keyboard interrupt handler\n"); // 등록 실패
                                                                // 메시지 출력
        return -EBUSY; // 오류 코드 반환
    }
    pr_info("Keyboard interrupt handler registered\n"); // 등록 성공 메시지 출력
    return 0;                                           // 성공
}

// 모듈 종료 함수
static void __exit my_exit(void) {
    free_irq(1, &my_device); // 인터럽트 핸틀러 함수 해제
    pr_info("Keyboard interrupt handler unregistered\n"); // 핸들러 함수 해제
                                                          // 메시지 출력
}

module_init(my_init);
module_exit(my_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("RustWithLinux");
MODULE_DESCRIPTION("Keyboard Interrupt Handler Example Kernel Module");
