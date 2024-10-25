//asc --stackSize=64512 main.ts --lowMemoryLimit --initialMemory=2 --maximumMemory=2 --memoryBase=1024 -o main.wasm && wasm2wat main.wasm
//fwi-serial -m 1 -s main.wasm -fn /scripts/as.wasm -w as
import { waitms, printInt, setBoardLED } from "./wiliwasm";
import { String } from "~lib/string";


export function _start(): void {
    waitms(1000);
    //let s = "Hello World!\n";
    //let s = String.UTF8.encode("Hello World!\n", true).toString();
    //let s: ArrayBuffer = String.UTF8.encode("Hello World!\n", true);
    //printInt("Hello World!\n\0", 0, 0, 0);

    for (let i = 0; i < 25; i++) {
        setBoardLED(0, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(1, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(2, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(3, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(4, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(5, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
        setBoardLED(6, 0x30, 0x30, 0x30, 500, 3);
        waitms(50);
    }

}