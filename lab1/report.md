## distributed computing lab 1 report
by: kalev kalda sikes
id: 100425828

### task 2 results

One can plainly see that the main causes are messages per second and message size. These two variables have a multiplicative effect on the package loss. This makes it apparent that byte-rate in the transfer is the main cause of packet loss.

full log dump from program running against a server 30 ms away:
```
starting test id 00, mps: 0064, msize: 00064
giving replies some time
reply count: 510
done waiting for replies
starting test id 01, mps: 0064, msize: 00256
giving replies some time
reply count: 1022
done waiting for replies
starting test id 02, mps: 0064, msize: 01024
giving replies some time
reply count: 1533
done waiting for replies
starting test id 03, mps: 0064, msize: 04096
giving replies some time
reply count: 2045
done waiting for replies
starting test id 04, mps: 0064, msize: 08192
giving replies some time
reply count: 2556
done waiting for replies
starting test id 05, mps: 0064, msize: 16384
giving replies some time
reply count: 3067
done waiting for replies
starting test id 06, mps: 0128, msize: 00064
giving replies some time
reply count: 3578
done waiting for replies
starting test id 07, mps: 0128, msize: 00256
giving replies some time
reply count: 4089
done waiting for replies
starting test id 08, mps: 0128, msize: 01024
giving replies some time
reply count: 4601
done waiting for replies
starting test id 09, mps: 0128, msize: 04096
giving replies some time
reply count: 5113
done waiting for replies
starting test id 10, mps: 0128, msize: 08192
giving replies some time
reply count: 5625
done waiting for replies
starting test id 11, mps: 0128, msize: 16384
giving replies some time
reply count: 5708
done waiting for replies
starting test id 12, mps: 0256, msize: 00064
giving replies some time
reply count: 6220
done waiting for replies
starting test id 13, mps: 0256, msize: 00256
giving replies some time
reply count: 6731
done waiting for replies
starting test id 14, mps: 0256, msize: 01024
giving replies some time
reply count: 7243
done waiting for replies
starting test id 15, mps: 0256, msize: 04096
giving replies some time
reply count: 7755
done waiting for replies
starting test id 16, mps: 0256, msize: 08192
giving replies some time
reply count: 7837
done waiting for replies
starting test id 17, mps: 0256, msize: 16384
giving replies some time

done waiting for replies
starting test id 18, mps: 1024, msize: 00064
giving replies some time
reply count: 8364
done waiting for replies
starting test id 19, mps: 1024, msize: 00256
giving replies some time
reply count: 8876
done waiting for replies
starting test id 20, mps: 1024, msize: 01024
giving replies some time
reply count: 9388
done waiting for replies
starting test id 21, mps: 1024, msize: 04096
giving replies some time
reply count: 9561
done waiting for replies
starting test id 22, mps: 1024, msize: 08192
giving replies some time

done waiting for replies
starting test id 23, mps: 1024, msize: 16384
giving replies some time

done waiting for replies
result for test id: 00 mps: 0064, msize: 00064 :: 510 / 512 ( 99.61 % )
result for test id: 01 mps: 0064, msize: 00256 :: 512 / 512 ( 100.00 % )
result for test id: 02 mps: 0064, msize: 01024 :: 511 / 512 ( 99.80 % )
result for test id: 03 mps: 0064, msize: 04096 :: 512 / 512 ( 100.00 % )
result for test id: 04 mps: 0064, msize: 08192 :: 511 / 512 ( 99.80 % )
result for test id: 05 mps: 0064, msize: 16384 :: 511 / 512 ( 99.80 % )
result for test id: 06 mps: 0128, msize: 00064 :: 511 / 512 ( 99.80 % )
result for test id: 07 mps: 0128, msize: 00256 :: 511 / 512 ( 99.80 % )
result for test id: 08 mps: 0128, msize: 01024 :: 512 / 512 ( 100.00 % )
result for test id: 09 mps: 0128, msize: 04096 :: 512 / 512 ( 100.00 % )
result for test id: 10 mps: 0128, msize: 08192 :: 512 / 512 ( 100.00 % )
result for test id: 11 mps: 0128, msize: 16384 :: 083 / 512 ( 16.21 % )
result for test id: 12 mps: 0256, msize: 00064 :: 512 / 512 ( 100.00 % )
result for test id: 13 mps: 0256, msize: 00256 :: 511 / 512 ( 99.80 % )
result for test id: 14 mps: 0256, msize: 01024 :: 512 / 512 ( 100.00 % )
result for test id: 15 mps: 0256, msize: 04096 :: 512 / 512 ( 100.00 % )
result for test id: 16 mps: 0256, msize: 08192 :: 082 / 512 ( 16.02 % )
result for test id: 17 mps: 0256, msize: 16384 :: 015 / 512 ( 02.93 % )
result for test id: 18 mps: 1024, msize: 00064 :: 512 / 512 ( 100.00 % )
result for test id: 19 mps: 1024, msize: 00256 :: 512 / 512 ( 100.00 % )
result for test id: 20 mps: 1024, msize: 01024 :: 512 / 512 ( 100.00 % )
result for test id: 21 mps: 1024, msize: 04096 :: 173 / 512 ( 33.79 % )
result for test id: 22 mps: 1024, msize: 08192 :: 034 / 512 ( 06.64 % )
result for test id: 23 mps: 1024, msize: 16384 :: 012 / 512 ( 02.34 % )
```
