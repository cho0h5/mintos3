#include "ModeSwitch.h"
#include "Page.h"
#include "Rusty.h"
#include "Types.h"

extern int add(int a, int b);

void kCopyKernel64ImageTo2Mbyte();
void copy_kernel64_image_to_2mbyte();

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
  kInitializePageTables();
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

void copy_kernel64_image_to_2mbyte() {
  WORD wKernel32SectorCount, wTotalKernelSectorCount;
  DWORD *pdwSourceAddress, *pdwDestinationAddress;
  int i;

  wTotalKernelSectorCount = *((WORD *)0x7C05);
  wKernel32SectorCount = *((WORD *)0x7C07);

  pdwSourceAddress = (DWORD *)(0x10000 + (wKernel32SectorCount * 512));
  pdwDestinationAddress = (DWORD *)0x200000;
  for (i = 0; i < 512 * (wTotalKernelSectorCount - wKernel32SectorCount) / 4;
       i++) {
    *pdwDestinationAddress = *pdwSourceAddress;
    pdwDestinationAddress += 1;
    pdwSourceAddress += 1;
  }
}
