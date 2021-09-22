# Collatz Conjecture on Rust #

### Overview
The Collatz conjecture is a conjecture in mathematics that concerns sequences. It most know as the 3n+1 Problem.

This Repo is an basic implementation to save every iteration in a json file (Because sometimes personally i play with this)


### Output
The output is a JSON File for Every Iteration. 
One Iteration have N results called *batchs*
Also I save the result tree for every result (If you wanna Graph It gonna be helpful)

This is the JSON struct
```json
[
	{
	"number":11,
	"steps":14,
	"historical":[11,34,17,52,26,13,40,20,10,5,16,8,4,2,1]
	}
]
```

Keys:
- number: is the evaluted number
- steps: the n steps that takes to be 1
- historical: the result vector for every iteration

### Run it

Check this repl [Link](https://replit.com/@migeruj/Collatz?v=1)
