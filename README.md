# rust-lisp

A toy [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)) implementation.
It is intended to run interpreted as a kind of fictional computer.

![CI](https://github.com/dchiquito/rust-lisp/actions/workflows/testing.yml/badge.svg)

# Brainstorming
* For simplicities sake, computers are single threaded and single processed, so no concurrency within a computer whatsoever.
  This makes a single computer a contained, functional unit.
* How can multiple computers interact, or utilize a shared resource (file system, network, etc.)?
  * If a computer is currently running something, it is locked.
  * An interface can feed the computer an atom/list/command/program to run, which locks the computer until the computation is complete and the result is returned.
  * An interface can be a user terminal, a network connection, or some other hardware peripheral.
  * A computer can send some kind of syscall to any interface as well.
  * While a computer is blocking on an outgoing request, it is _unlocked_ (wow!)
  * A file server would work like this:
    * user types something into terminal
    * terminal sends command to computer
    * computer sends command to networked server
    * server sends command to disk drive
    * disk drive does read/write, whatever. Anything else can use the computer/network/file server while this is happening, but the disk drive is locked.
    * Everything returns up the call stack.
  * Maybe some computers do not unlock, just to make it easier to reason about concurrent modification.
  * Interface requests can choose whether to fail on lock or block on lock.
  * Memory is frozen while blocked on I/O, so a computer can only handle so many concurrent processes.
* computer memory is finite and consumable, but also managed and implicitly defragmented. An error is thrown if memory is ever exceeded.
* A computer could potentiall have multiple cores to allow concurrency.
  * Cores would share memory and interfaces.
  * Incoming interface requests would select an idle core, or throw an appropriate blocking error if none are available.
  * Outgoing interfaces are shared between cores, but for example multiple cores cannot access a file system simultaneously. One would block the other.
  * This does make things harder to reason about so no rush.
* A computer runs at a fixed number of ticks/second.
  * system calls (math, I/O, standard library, etc.) take some well defined number of ticks (not necessarily 1)
  * Calling a function takes at least 1 tick
    * It could hypothetically be more complicated, like 1 + the number of arguments
* There is a global scheduler that runs all simulated computers and keeps them throttled to their tick rate.
  * If there is too much for the simulation host to handle, some of the computers encounter "performance issues".
  * Running processes are kept in some randomized ordering so that the same processes consistently encounter performance issues.
  * Hypothetically, the scheduler should drive every computer one tick forward, then idle for the rest of the time to simulate the tick rate.
    However, the scheduler can cheat and use that idle time to drive computers that are doing internal calculations.
    Any I/O requests necessitate waiting for the computer to resynchronize so that any other slower computers have a chance to finish interacting with any shared resources.