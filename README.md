# Rust Atomics and Locks


### Rust Concurrency
Long before multi-core processors were commonplace, operating systems allowed for a single computer to run many programs concurrently. This is achieved by rapidly switching between processes, allowing each to repeatedly make a little bit of progress, one by one. Nowadays, virtually all our computers and even our phones and watches have processors with multiple cores, which can truly execute multiple processes in parallel.

Operating systems isolate processes from each other as much as possible, allowing a program to do its thing while completely unaware of what any other processes are doing. For example, a process cannot normally access the memory of another process, or communicate with it in any way, without asking the operating system’s kernel first.

However, a program can spawn extra threads of execution, as part of the same process. Threads within the same process are not isolated from each other. Threads share memory and can interact with each other through that memory.


> **Note from Rust in Action Preface:**
> 
> *Rust is not reserved for a select group of experts. It is a tool that’s available for everyone*


[Rust Atomics and Locks](https://marabos.nl/atomics/).
