# bellman scaling benchmarks

This repository summarises our analysis on the scaling behaviour of the bellman zkSNARK backend for larger constraint circuits.

We use the blake2s hashing circuit as a basis and conduct runs for several pre-image sizes:

| ﻿NumConstraints 	| ByteInput 	| MinMemoryPerExecutorGB 	| RunTimeParamGenSec 	| RunTimeProverSec 	| ParameterMB 	|
|----------------	|------------	|------------------------	|--------------------	|------------------	|-------------	|
| 64K            	| 192        	| 0.1                    	| 9.5                	| 0.46             	| 32          	|
| 128K           	| 384        	| 0.18                   	| 18.95              	| 0.83             	| 63          	|
| 256K           	| 768        	| 0.27                   	| 43.8               	| 2.6              	| 150         	|
| 512K           	| 1536       	| 0.758                  	| 87.6               	| 5.3              	| 300         	|
| 1M             	| 3072       	| 1.5                    	| 174.5              	| 10.57            	| 601         	|
| 2M             	| 6144       	| 2.9                    	| 348.7              	| 20               	| 1200        	|
| 4M             	| 12288      	| 5.8                    	| 696.3              	| 40.4             	| 2400        	|
| 8M             	| 24576      	| 11.5                   	| 1400.8             	| 82.43            	| 4700        	|
| 16M            	| 49152      	| 23.1                   	| 2789.5             	| 167.1            	| 9400        	|

We use the following system config to run the benchmark program:

```bash
 $ inxi -C
CPU: 6 core Intel Core i5-8400 (-MCP-) cache: 9216 KB
clock speeds: max: 4000 MHz 1: 800 MHz 2: 800 MHz 3: 800 MHz 4: 800 MHz 5: 800 MHz 6: 800 MHz
```