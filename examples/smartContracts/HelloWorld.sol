pragma solidity ^0.5.3;

contract HelloWorld {
 function getInt () public pure returns (uint) {
   return 4251627384;
 }
 function getString () public pure returns (string memory) {
   return "John says hi";
 }
 function sum (uint a, uint b) public pure returns (uint) {
   return a + b;
}
}