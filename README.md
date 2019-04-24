# bellman scaling benchmarks

This repository summarises our analysis on the scaling behaviour of the bellman zkSNARK backend for larger constraint circuits.

We use the blake2s hashing circuit as a basis and conduct runs for several pre-image sizes:

## Groth16

| NumConstraints 	| ByteInput 	| MinMemoryPerExecutorGB 	| RunTimeParamGenSec 	| RunTimeProverSec 	| ParameterMB 	|
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

## Sonic

| NumConstraints 	| Byte input 	| MinMemoryPerExecutorGB 	| RunTimeParamGenSec 	| RunTimeProverSec 	| RunTimeCreateAdvice 	| RunTimeGenAggregate5 	| RunTimeVerify1Proof 	| RunTimeVerify5Naive 	| RunTimeVerify5Aggregated 	|
|----------------	|------------	|------------------------	|--------------------	|------------------	|---------------------	|----------------------	|---------------------	|---------------------	|--------------------------	|
| 64K            	| 192        	| 0.5                    	| 84                 	| 4                	| 1.2                 	| 6.5                  	| 147ms               	| 525ms               	| 170ms                    	|
| 128K           	| 384        	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 12.5                 	| 277ms               	| 988ms               	| 303ms                    	|
| 256K           	| 768        	| 1.8                    	| 341                	| 14.8             	| 4.2                 	| 23.3                 	| 539ms               	| 1.98s               	| 563ms                    	|
| 512K           	| 1536       	| 3.6                    	| 683                	| 28.9             	| 8.2                 	| 42.8                 	| 1                   	| 3.9                 	| 1.1                      	|
| 1M             	| 3072       	| 7.1                    	| 1365               	| 57               	| 15.9                	| 83.6                 	| 2.1                 	| 7.8                 	| 2.1                      	|
| 2M             	| 6144       	| 14.3                   	| 2735               	| 112              	| 31                  	| 162                  	| 4.2                 	| 15.6                	| 4.2                      	|

Some additional runs with varying batch size:


| NumConstraints 	| Byte input 	| BatchSize 	| MinMemoryPerExecutorGB 	| RunTimeParamGenSec 	| RunTimeProverSec 	| RunTimeCreateAdvice 	| RunTimeGenAggregate 	| RunTimeVerify1 	| RunTimeVerifyBatch 	| RunTimeVerifyBatchAggregated 	|
|----------------	|------------	|-----------	|------------------------	|--------------------	|------------------	|---------------------	|---------------------	|---------------------	|--------------------	|------------------------------	|
| 128K           	| 384        	| 3         	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 9                   	| 285ms               	| 646ms              	| 300ms                        	|
| 128K           	| 384        	| 5         	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 12.5                	| 281ms               	| 1                  	| 300ms                        	|
| 128K           	| 384        	| 10        	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 20.32               	| 281ms               	| 1.87               	| 295ms                        	|
| 128K           	| 384        	| 20        	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 36                  	| 281ms               	| 3.62               	| 295ms                        	|
| 128K           	| 384        	| 40        	| 0.9                    	| 170                	| 7.9              	| 2.4                 	| 68.7                	| 281ms               	| 7.1                	| 300ms                        	|

We use the following system config to run the benchmark program:

```bash
 $ inxi -C
CPU: 6 core Intel Core i5-8400 (-MCP-) cache: 9216 KB
clock speeds: max: 4000 MHz 1: 800 MHz 2: 800 MHz 3: 800 MHz 4: 800 MHz 5: 800 MHz 6: 800 MHz
```