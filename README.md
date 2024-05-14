## Code runner
Code runner is a simple online runner written in rust. Use docker behind to execute the code


## Run server
```shell
cargo watch -x run
```

## Supported langs
* Rust
* Python
* Javascript

## Runner request:
Make a post request:
```
POST /api/runner/execute
```
Submit the code in a json body payload

```json
{ 
  "id": "....",
  "lang": "...",
  "solution_code": "...",
  "main_code": "....",
  "inputs": [{
    "args": "[1,3]",
    "expected_result": "4"
  }]
}
```
| Property | Type | Description | 
| --- | --- | --- |
| id | string | Code submission id (could be an any string) |
| lang | string | Allowed values: `Rust`, `Javascript`, `Python` |
| solution_code | string | Code containg the algorithm solution |
| main_code | string | Code that contains the main program code to read the arguments and pass them to the solution code. |
| inputs | Object | The solution code inputs (test cases)


### Example request
Running a program that sum two numbers:

solution_code:

```js
const sum = (a, b) => {
    return a + b;
};

exports.sum = sum;
```

main code:

```js
const solution = require('./solution'); 

const main = () => { 
    const args = process.argv.slice(2); 
    const params = JSON.parse(args); 

    console.log(solution.sum(...params)); 
}; 

main();
```
### Body payload:

```json
{ 
  "id": "sum_two_numbers",
  "lang": "Javascript",
  "solution_code": "const sum = (a, b) => {return a + b;};exports.sum = sum;",
  "main_code": "const solution = require('./solution'); const main = () => { const args = process.argv.slice(2); const params = JSON.parse(args); const result = solution.sum(...params); console.log(result); }; main();",
  "inputs": [{
    "args": "[1,3]",
    "expected_result": "4"
  },{
    "args": "[2,3]",
    "expected_result": "5"
  },
  {
    "args": "[2,6]",
    "expected_result": "8"
  }]
}
```
### Output

```json
{
  "is_success": true,
  "message": "Submission was executed",
  "results": [
    {
      "input": "[1,3]",
      "output": "4",
      "expected_result": "4"
    },
    {
      "input": "[2,3]",
      "output": "5",
      "expected_result": "5"
    },
    {
      "input": "[2,6]",
      "output": "8",
      "expected_result": "10"
    }
  ]
}
```
