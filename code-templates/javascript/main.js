const solution = require('./solution'); 

const main = () => { 
    const args = process.argv.slice(2); 
    const params = JSON.parse(args); 
    const result = solution.sum(...params); 
    console.log(result); 
}; 

main();