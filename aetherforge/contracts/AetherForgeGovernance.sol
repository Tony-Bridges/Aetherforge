// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AetherForgeGovernance {
    // --- Storage ---

    // Store votes for each proposal by address
    mapping(uint256 => mapping(address => uint256)) public votes;

    // Store total votes for each proposal
    mapping(uint256 => uint256) public totalVotes;

    // Store list of voters (using a mapping to avoid duplicates)
    mapping(address => bool) public voters;

    // Current proposal count
    uint256 public proposalCount;

    // Store proposals with their parameters
    struct Proposal {
        uint256 targetBlockTime;
        uint256 argon2MemCost;
        bool executed; // Track if the proposal has been executed
    }

    // Store proposals by proposal ID
    mapping(uint256 => Proposal) public proposals;

    // --- Events ---
    event ProposalCreated(uint256 proposalId, uint256 targetBlockTime, uint256 argon2MemCost);
    event VoteCast(address voter, uint256 proposalId, uint256 amount);
    event ProposalExecuted(uint256 proposalId);

    // --- Functions ---

    // Function to create a new proposal
    function createProposal(
        uint256 _targetBlockTime,
        uint256 _argon2MemCost
    ) public returns (uint256) {
        proposalCount += 1;
        proposals[proposalCount] = Proposal({
            targetBlockTime: _targetBlockTime,
            argon2MemCost: _argon2MemCost,
            executed: false
        });
        emit ProposalCreated(proposalCount, _targetBlockTime, _argon2MemCost);
        return proposalCount;
    }

    // Function for users to cast their votes on a proposal
    function vote(uint256 proposalId, uint256 amount) public {
        require(proposalId <= proposalCount, "Invalid proposal ID");
        require(!proposals[proposalId].executed, "Proposal already executed");

        // Deduct voting tokens (implement token integration in a real scenario)
        // tokenContract.transferFrom(msg.sender, address(this), amount);

        votes[proposalId][msg.sender] += amount;
        totalVotes[proposalId] += amount;
        voters[msg.sender] = true; // Mark the voter
        emit VoteCast(msg.sender, proposalId, amount);
    }

    // Function to execute a proposal (if it has enough votes)
    function executeProposal(uint256 proposalId) public {
        require(proposalId <= proposalCount, "Invalid proposal ID");
        require(!proposals[proposalId].executed, "Proposal already executed");
        require(totalVotes[proposalId] > 1000, "Not enough votes");

        Proposal storage proposal = proposals[proposalId];

        // Update parameters (replace with actual functions/variables in your system)
        // setTargetBlockTime(proposal.targetBlockTime);
        // setArgon2MemCost(proposal.argon2MemCost);

        proposal.executed = true; // Mark the proposal as executed
        emit ProposalExecuted(proposalId);
    }

    // --- Helper Functions ---

    // Function to check if a voter has voted on a proposal
    function hasVoted(uint256 proposalId, address voter) public view returns (bool) {
        return votes[proposalId][voter] > 0;
    }

    // Function to get the total votes for a proposal
    function getTotalVotes(uint256 proposalId) public view returns (uint256) {
        return totalVotes[proposalId];
    }
}