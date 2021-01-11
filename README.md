# prfactorization-rs
Program that factorize any number in primes written in Rust.

## Known Issues:

### Can't find prime for low numbers.
#### Example:
*Input:* 26;  
*Output:* [2];  
But we now that prime factorization of 26 is 2 * 13.  
Reason of it is algorithm optimization: algorithm complexity is O(√n), it was done for big numbers, because this program was firstly coded to solve problem that needed to operate with big numbers.


### Can't print duplicates.
#### Example:
*Input:* 24;  
*Output:* [2, 3];  
But we now that prime factorization of 24 is 2³ * 3.  
There's no reason, really. I will fix this soon.

## Half-fixed, but freezing.
