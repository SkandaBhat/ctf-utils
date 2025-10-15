// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {CTHelpers} from "./CTHelpers.sol";

contract PositionsTest is Test {
    function test_fuzz_position_ids(address _collateral, bytes32 _conditionId, uint256 _outcomeIndex) public {
        uint256 outcomeIndex = _outcomeIndex % 2;

        string[] memory inputs = new string[](4);
        inputs[0] = "./target/debug/ctf-utils-cli";
        inputs[1] = vm.toString(_collateral);
        inputs[2] = vm.toString(_conditionId);
        inputs[3] = vm.toString(outcomeIndex);

        bytes memory res = vm.ffi(inputs);
        uint256 result = abi.decode(res, (uint256));

        uint256 indexSet = 1 << outcomeIndex;
        bytes32 collectionId = CTHelpers.getCollectionId(bytes32(0), _conditionId, indexSet);
        uint256 positionId = CTHelpers.getPositionId(_collateral, collectionId);

        assertEq(result, positionId);
    }
}
