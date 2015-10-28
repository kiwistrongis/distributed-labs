# Distributed Computing Lab 3 Report
By: Kalev Kalda Sikes
Id: 100425828

## Task One
### Question One
When I made the input file 4953313 lines long, the program's cpu usage peaked at 76% for a few seconds.

### Question Two
The lines in the output file would (likely) in a slighty different order than the input file.

## Task Two
### Question One
All measurements peaked at 100%.

### Question Two
The optimal setting for a 4-core machine is 5 workers - one producer, three filterers, and one consumer. This ensures that until and after the producer finishes, the maximum amount of cores are used until the producer's queue is empty.

## General Question
Tcp. It essentially functions as a remote message queue.
