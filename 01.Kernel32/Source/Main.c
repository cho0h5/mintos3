#include "ModeSwitch.h"
#include "Rusty.h"
#include "Types.h"

extern int add(int a, int b);

void kCopyKernel64ImageTo2Mbyte(void);

void Main() {
  kPrintString(0, 3, "C Language Kernel Start.....................[Pass]");
  kPrintString(0, 4, "Mininum Memory Size Check...................[    ]");
  if (kIsMemoryEnough() == FALSE) {
    kPrintString(45, 4, "Fail");
    kPrintString(0, 5, "Not Enough Memory.");
    while (1)
      ;
  }
  kPrintString(45, 4, "Pass");

  kPrintString(0, 5, "IA-32e Kernel Area Initialization...........[    ]");
  if (kInitializeKernel64Area() == FALSE) {
    kPrintString(45, 5, "Fail");
    kPrintString(0, 6, "Kernel Area Initialization Fail!");
    while (1)
      ;
  }
  kPrintString(45, 5, "Pass");

  kPrintString(0, 6, "IA-32e Page Tables Initialize...............[    ]");
  k_initialize_page_tables();
  kPrintString(45, 6, "Pass");

  kPrintString(0, 7,
               "Processor Vendor String.....................[            ]");
  print_cpu_manufacturer();

  kPrintString(0, 8, "64bit Mode Support Check....................[    ]");
  if (is_support_64()) {
    kPrintString(45, 8, "Pass");
  } else {
    kPrintString(45, 8, "Fail");
    while (1)
      ;
  }

  kPrintString(0, 9, "Copy IA-32e Kernel To 2M Address............[    ]");
  copy_kernel64_image_to_2mbyte();
  kPrintString(45, 9, "Pass");

  kPrintString(0, 10, "Switch to IA-32e Mode");
  kSwitchAndExecute64bitKernel();

  while (1)
    ;
}
