### Getting Started (In Rust directory)
---------------
**Env**
  - OS: Arch Linux 
  -  Editor: Doom Emac
  - Other: gdb[gef]
  
  ---------------
  **Setup**

   

    cargo new <your name project>


    
  ---------------
  **Run** for dev [unoptimized]
 compile only not create binary file  

    cargo check 
compile create binary file and run 

    cargo run
compile and create binary file in directory target

    cargo build

for Production **optimized**

    cargo build --release
**Debug with gdb**
goto directory 

    taget/debug

and execute your file with gdb

    gdb <your file>

then 

    b main = create break point
    r = start 
    n = next
    i local = show local in memory - hex
    xinfo &<your varialbe> = show address of variable
    s = into function if n will step over function
    x <address>= show value hex in memory use address from xinfo
    x /d <address> = decimal
    c = continue
    
    
