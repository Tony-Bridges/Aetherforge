const AetherForgeRewards = artifacts.require("AetherForgeRewards");
const AetherForgeGovernance = artifacts.require("AetherForgeGovernance");

module.exports = function (deployer) {
  deployer.deploy(AetherForgeRewards);
  deployer.deploy(AetherForgeGovernance);
};