// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Strings.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

struct FlightCoord {
    string _lat;
    string _long;
}

contract EFTemplate{
    string[] public flight_coords = ["50.4504:30.5245"];

    address payable public owner;
    uint64 public maxOweAmount;

    event FlightTransfered(address _to, uint256 _amount);
    event FlightCoordAdded(string _lat, string _long);

    constructor(uint64 _maxOweAmount) {
        owner = payable(msg.sender);
        maxOweAmount = _maxOweAmount;
    }

    modifier onlyOwner {
        require(msg.sender == owner);
        _;
    }

    modifier notTheOwner {
        require(msg.sender != owner);
        _;
    }

    modifier priceMatched {
        require(msg.value >= maxOweAmount);
        _;
    }

    function get_flight_coords() public view returns (string[] memory) {
        return flight_coords;
    }

    function verify_flight_is_alive() public view returns (bool) {
        return flight_coords.length > 0;
    }

    function push_flight_coord(string memory _lat, string memory _long) public {
        flight_coords.push(string(abi.encodePacked(_lat, ":", _long)));
        emit FlightCoordAdded(_lat, _long);
    }

    function owe_flight(uint64 _newOweAmount) payable public priceMatched notTheOwner returns (bool) {
        owner.transfer(msg.value);
        owner = payable(msg.sender);
        maxOweAmount = _newOweAmount;
        emit FlightTransfered(msg.sender, msg.value);
        return true;
    }
}
