# runtime/ops

So that runtime/application functions are not confused, each are feature flagged.

To keep things safe, an op is allocated by the application and the pointer is sent to the runtime. The runtime should then read the memory, perform the requested operation, and mutate the op to provide a response. During this process, a few rules must be followed.

1. The runtime must not swap to or away from any enum variants containing pointers. Only the application can allocate pointers and free the data behind them.
2. The runtime should only respond by writing memory at pointers allocated by the application. It may not give a response that contains any newly allocated memory.
3. The runtime must not attempt to free the op memory in any way.

Another thing to point out... Effort is made to ensure that all unsafe code is contained in this crate so that neither the runtime or the application are required to write unsafe code in order to allocate or mutate ops. By "safety", the following is meant:

1. Assuming the runtime is safe, the application is guaranteed to be safe.
2. If the application is not safe, or even intentionally malicious, the runtime will not crash or produce undefined behavior, although, it may inadvertently cause the application to crash or produce undefined behavior.
