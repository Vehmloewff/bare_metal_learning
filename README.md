# Agincourt

Research into an operating system with a focus on security and modularity.

This idea here is that the operating system does almost nothing. Almost every feature is implemented by applications.

## Security

Applications run in a sandboxed environment where each application receives exclusive read and write access to resizable chunks of storage and memory.

Applications can then create certificates, which allow other applications to read or write sections of this exclusive memory.

## Modularity

Instead of process execution with arguments and environment variables, applications expose functions, which can be called by other applications directly or through a versioned interfaces.

### OS Operations

All applications interact with the OS through a set of operations. Because the OS does so little, this list of operations is rather small.
