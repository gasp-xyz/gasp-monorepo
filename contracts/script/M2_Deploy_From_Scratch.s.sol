// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {AVSDirectory} from "@eigenlayer/contracts/core/AVSDirectory.sol";
import {DelegationManager} from "@eigenlayer/contracts/core/DelegationManager.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {Slasher} from "@eigenlayer/contracts/core/Slasher.sol";
import {IStrategy, StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";
import {IETHPOSDeposit} from "@eigenlayer/contracts/interfaces/IETHPOSDeposit.sol";
import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {EigenPod} from "@eigenlayer/contracts/pods/EigenPod.sol";
import {EigenPodManager} from "@eigenlayer/contracts/pods/EigenPodManager.sol";
import {StrategyBase, StrategyBaseTVLLimits} from "@eigenlayer/contracts/strategies/StrategyBaseTVLLimits.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {UpgradeableBeacon} from "@openzeppelin/contracts/proxy/beacon/UpgradeableBeacon.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Test, Vm} from "forge-std/Test.sol";

// # To load the variables in the .env file
// source .env

// # To deploy and verify our contract
// forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY \
//     --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
contract M2Deployer is Script, Test {
    address public constant HEVM_ADDRESS = address(bytes20(uint160(uint256(keccak256("hevm cheat code")))));
    Vm public cheats = Vm(HEVM_ADDRESS);

    // struct used to encode token info in config file
    struct StrategyConfig {
        uint256 maxDeposits;
        uint256 maxPerDeposit;
        address tokenAddress;
        string tokenSymbol;
    }

    string public deployConfigPath;

    // EigenLayer Contracts
    ProxyAdmin public eigenLayerProxyAdmin;
    PauserRegistry public eigenLayerPauserReg;
    Slasher public slasher;
    Slasher public slasherImplementation;
    DelegationManager public delegation;
    DelegationManager public delegationImplementation;
    StrategyManager public strategyManager;
    StrategyManager public strategyManagerImplementation;
    RewardsCoordinator public rewardsCoordinator;
    RewardsCoordinator public rewardsCoordinatorImplementation;
    AVSDirectory public avsDirectory;
    AVSDirectory public avsDirectoryImplementation;
    EigenPodManager public eigenPodManager;
    EigenPodManager public eigenPodManagerImplementation;
    UpgradeableBeacon public eigenPodBeacon;
    EigenPod public eigenPodImplementation;
    StrategyBase public baseStrategyImplementation;

    EmptyContract public emptyContract;

    address public executorMultisig;
    address public operationsMultisig;
    address public pauserMultisig;

    // the ETH2 deposit contract -- if not on mainnet, we deploy a mock as stand-in
    IETHPOSDeposit public ethPOSDeposit;

    // strategies deployed
    StrategyBaseTVLLimits[] public deployedStrategyArray;

    // IMMUTABLES TO SET
    uint64 public GOERLI_GENESIS_TIME = 1616508000;

    // OTHER DEPLOYMENT PARAMETERS
    uint256 public STRATEGY_MANAGER_INIT_PAUSED_STATUS;
    uint256 public SLASHER_INIT_PAUSED_STATUS;
    uint256 public DELEGATION_INIT_PAUSED_STATUS;
    uint256 public EIGENPOD_MANAGER_INIT_PAUSED_STATUS;
    uint256 public REWARDS_COORDINATOR_INIT_PAUSED_STATUS;

    // RewardsCoordinator
    uint32 public REWARDS_COORDINATOR_MAX_REWARDS_DURATION;
    uint32 public REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH;
    uint32 public REWARDS_COORDINATOR_MAX_FUTURE_LENGTH;
    uint32 public REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP;
    address public REWARDS_COORDINATOR_UPDATER;
    uint32 public REWARDS_COORDINATOR_ACTIVATION_DELAY;
    uint32 public REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS;
    uint32 public REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS;

    // one week in blocks -- 50400
    uint32 public STRATEGY_MANAGER_INIT_WITHDRAWAL_DELAY_BLOCKS;
    uint256 public DELEGATION_WITHDRAWAL_DELAY_BLOCKS;

    function run(string memory configFile) external {
        // read and log the chainID
        uint256 chainId = block.chainid;
        emit log_named_uint("You are deploying on ChainID", chainId);

        // READ JSON CONFIG DATA
        deployConfigPath = string(bytes(string.concat("script/", configFile)));
        string memory config_data = vm.readFile(deployConfigPath);
        // bytes memory parsedData = vm.parseJson(config_data);

        STRATEGY_MANAGER_INIT_PAUSED_STATUS = stdJson.readUint(config_data, ".strategyManager.init_paused_status");
        SLASHER_INIT_PAUSED_STATUS = stdJson.readUint(config_data, ".slasher.init_paused_status");
        DELEGATION_INIT_PAUSED_STATUS = stdJson.readUint(config_data, ".delegation.init_paused_status");
        DELEGATION_WITHDRAWAL_DELAY_BLOCKS = stdJson.readUint(config_data, ".delegation.init_withdrawal_delay_blocks");
        EIGENPOD_MANAGER_INIT_PAUSED_STATUS = stdJson.readUint(config_data, ".eigenPodManager.init_paused_status");
        REWARDS_COORDINATOR_INIT_PAUSED_STATUS = stdJson.readUint(config_data, ".rewardsCoordinator.init_paused_status");
        REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.CALCULATION_INTERVAL_SECONDS"));
        REWARDS_COORDINATOR_MAX_REWARDS_DURATION =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.MAX_REWARDS_DURATION"));
        REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.MAX_RETROACTIVE_LENGTH"));
        REWARDS_COORDINATOR_MAX_FUTURE_LENGTH =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.MAX_FUTURE_LENGTH"));
        REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.GENESIS_REWARDS_TIMESTAMP"));
        REWARDS_COORDINATOR_UPDATER = stdJson.readAddress(config_data, ".rewardsCoordinator.rewards_updater_address");
        REWARDS_COORDINATOR_ACTIVATION_DELAY =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.activation_delay"));
        REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.calculation_interval_seconds"));
        REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS =
            uint32(stdJson.readUint(config_data, ".rewardsCoordinator.global_operator_commission_bips"));

        STRATEGY_MANAGER_INIT_WITHDRAWAL_DELAY_BLOCKS =
            uint32(stdJson.readUint(config_data, ".strategyManager.init_withdrawal_delay_blocks"));

        // tokens to deploy strategies for
        StrategyConfig[] memory strategyConfigs;

        executorMultisig = stdJson.readAddress(config_data, ".multisig_addresses.executorMultisig");
        operationsMultisig = stdJson.readAddress(config_data, ".multisig_addresses.operationsMultisig");
        pauserMultisig = stdJson.readAddress(config_data, ".multisig_addresses.pauserMultisig");
        // load token list
        bytes memory strategyConfigsRaw = stdJson.parseRaw(config_data, ".strategies");
        strategyConfigs = abi.decode(strategyConfigsRaw, (StrategyConfig[]));

        require(executorMultisig != address(0), "executorMultisig address not configured correctly!");
        require(operationsMultisig != address(0), "operationsMultisig address not configured correctly!");

        // START RECORDING TRANSACTIONS FOR DEPLOYMENT
        vm.startBroadcast();

        // deploy proxy admin for ability to upgrade proxy contracts
        eigenLayerProxyAdmin = new ProxyAdmin();

        //deploy pauser registry
        {
            address[] memory pausers = new address[](3);
            pausers[0] = executorMultisig;
            pausers[1] = operationsMultisig;
            pausers[2] = pauserMultisig;
            eigenLayerPauserReg = new PauserRegistry(pausers, executorMultisig);
        }

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        emptyContract = new EmptyContract();
        delegation = DelegationManager(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), ""))
        );
        strategyManager = StrategyManager(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), ""))
        );
        avsDirectory = AVSDirectory(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), ""))
        );
        slasher =
            Slasher(address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), "")));
        eigenPodManager = EigenPodManager(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), ""))
        );
        rewardsCoordinator = RewardsCoordinator(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(eigenLayerProxyAdmin), ""))
        );

        // if on mainnet, use the ETH2 deposit contract address
        if (chainId == 1) {
            ethPOSDeposit = IETHPOSDeposit(0x00000000219ab540356cBB839Cbe05303d7705Fa);
            // if not on mainnet, deploy a mock
        } else {
            ethPOSDeposit = IETHPOSDeposit(stdJson.readAddress(config_data, ".ethPOSDepositAddress"));
        }
        eigenPodImplementation = new EigenPod(ethPOSDeposit, eigenPodManager, GOERLI_GENESIS_TIME);

        eigenPodBeacon = new UpgradeableBeacon(address(eigenPodImplementation));

        // Second, deploy the *implementation* contracts, using the *proxy contracts* as inputs
        delegationImplementation = new DelegationManager(strategyManager, slasher, eigenPodManager);
        strategyManagerImplementation = new StrategyManager(delegation, eigenPodManager, slasher);
        avsDirectoryImplementation = new AVSDirectory(delegation);
        slasherImplementation = new Slasher(strategyManager, delegation);
        eigenPodManagerImplementation =
            new EigenPodManager(ethPOSDeposit, eigenPodBeacon, strategyManager, slasher, delegation);
        rewardsCoordinatorImplementation = new RewardsCoordinator(
            delegation,
            strategyManager,
            REWARDS_COORDINATOR_CALCULATION_INTERVAL_SECONDS,
            REWARDS_COORDINATOR_MAX_REWARDS_DURATION,
            REWARDS_COORDINATOR_MAX_RETROACTIVE_LENGTH,
            REWARDS_COORDINATOR_MAX_FUTURE_LENGTH,
            REWARDS_COORDINATOR_GENESIS_REWARDS_TIMESTAMP
        );

        // Third, upgrade the proxy contracts to use the correct implementation contracts and initialize them.
        {
            IStrategy[] memory _strategies;
            uint256[] memory _withdrawalDelayBlocks;
            eigenLayerProxyAdmin.upgradeAndCall(
                TransparentUpgradeableProxy(payable(address(delegation))),
                address(delegationImplementation),
                abi.encodeWithSelector(
                    DelegationManager.initialize.selector,
                    executorMultisig,
                    eigenLayerPauserReg,
                    DELEGATION_INIT_PAUSED_STATUS,
                    DELEGATION_WITHDRAWAL_DELAY_BLOCKS,
                    _strategies,
                    _withdrawalDelayBlocks
                )
            );
        }
        eigenLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(strategyManager))),
            address(strategyManagerImplementation),
            abi.encodeWithSelector(
                StrategyManager.initialize.selector,
                executorMultisig,
                operationsMultisig,
                eigenLayerPauserReg,
                STRATEGY_MANAGER_INIT_PAUSED_STATUS
            )
        );
        eigenLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(slasher))),
            address(slasherImplementation),
            abi.encodeWithSelector(
                Slasher.initialize.selector, executorMultisig, eigenLayerPauserReg, SLASHER_INIT_PAUSED_STATUS
            )
        );
        eigenLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(avsDirectory))),
            address(avsDirectoryImplementation),
            abi.encodeWithSelector(AVSDirectory.initialize.selector, executorMultisig, eigenLayerPauserReg, 0)
        );
        eigenLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(eigenPodManager))),
            address(eigenPodManagerImplementation),
            abi.encodeWithSelector(
                EigenPodManager.initialize.selector,
                executorMultisig,
                eigenLayerPauserReg,
                EIGENPOD_MANAGER_INIT_PAUSED_STATUS
            )
        );
        eigenLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(rewardsCoordinator))),
            address(rewardsCoordinatorImplementation),
            abi.encodeWithSelector(
                RewardsCoordinator.initialize.selector,
                executorMultisig,
                eigenLayerPauserReg,
                REWARDS_COORDINATOR_INIT_PAUSED_STATUS,
                REWARDS_COORDINATOR_UPDATER,
                REWARDS_COORDINATOR_ACTIVATION_DELAY,
                REWARDS_COORDINATOR_GLOBAL_OPERATOR_COMMISSION_BIPS
            )
        );

        // deploy StrategyBaseTVLLimits contract implementation
        baseStrategyImplementation = new StrategyBaseTVLLimits(strategyManager);
        // create upgradeable proxies that each point to the implementation and initialize them
        for (uint256 i = 0; i < strategyConfigs.length; ++i) {
            deployedStrategyArray.push(
                StrategyBaseTVLLimits(
                    address(
                        new TransparentUpgradeableProxy(
                            address(baseStrategyImplementation),
                            address(eigenLayerProxyAdmin),
                            abi.encodeWithSelector(
                                StrategyBaseTVLLimits.initialize.selector,
                                strategyConfigs[i].maxPerDeposit,
                                strategyConfigs[i].maxDeposits,
                                IERC20(strategyConfigs[i].tokenAddress),
                                eigenLayerPauserReg
                            )
                        )
                    )
                )
            );
        }

        eigenLayerProxyAdmin.transferOwnership(executorMultisig);
        eigenPodBeacon.transferOwnership(executorMultisig);

        // STOP RECORDING TRANSACTIONS FOR DEPLOYMENT
        vm.stopBroadcast();

        // CHECK CORRECTNESS OF DEPLOYMENT
        _verifyContractsPointAtOneAnother(
            delegationImplementation,
            strategyManagerImplementation,
            slasherImplementation,
            eigenPodManagerImplementation,
            rewardsCoordinatorImplementation
        );
        _verifyContractsPointAtOneAnother(delegation, strategyManager, slasher, eigenPodManager, rewardsCoordinator);
        _verifyImplementationsSetCorrectly();
        _verifyInitialOwners();
        _checkPauserInitializations();
        _verifyInitializationParams();

        // WRITE JSON DATA
        string memory parent_object = "parent object";

        string memory deployed_strategies = "strategies";
        for (uint256 i = 0; i < strategyConfigs.length; ++i) {
            vm.serializeAddress(deployed_strategies, strategyConfigs[i].tokenSymbol, address(deployedStrategyArray[i]));
        }
        string memory deployed_strategies_output = strategyConfigs.length == 0
            ? ""
            : vm.serializeAddress(
                deployed_strategies,
                strategyConfigs[strategyConfigs.length - 1].tokenSymbol,
                address(deployedStrategyArray[strategyConfigs.length - 1])
            );

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "eigenLayerProxyAdmin", address(eigenLayerProxyAdmin));
        vm.serializeAddress(deployed_addresses, "eigenLayerPauserReg", address(eigenLayerPauserReg));
        vm.serializeAddress(deployed_addresses, "slasher", address(slasher));
        vm.serializeAddress(deployed_addresses, "slasherImplementation", address(slasherImplementation));
        vm.serializeAddress(deployed_addresses, "delegationManager", address(delegation));
        vm.serializeAddress(deployed_addresses, "delegationImplementation", address(delegationImplementation));
        vm.serializeAddress(deployed_addresses, "avsDirectory", address(avsDirectory));
        vm.serializeAddress(deployed_addresses, "avsDirectoryImplementation", address(avsDirectoryImplementation));
        vm.serializeAddress(deployed_addresses, "strategyManager", address(strategyManager));
        vm.serializeAddress(deployed_addresses, "strategyManagerImplementation", address(strategyManagerImplementation));
        vm.serializeAddress(deployed_addresses, "eigenPodManager", address(eigenPodManager));
        vm.serializeAddress(deployed_addresses, "eigenPodManagerImplementation", address(eigenPodManagerImplementation));
        vm.serializeAddress(deployed_addresses, "rewardsCoordinator", address(rewardsCoordinator));
        vm.serializeAddress(
            deployed_addresses, "rewardsCoordinatorImplementation", address(rewardsCoordinatorImplementation)
        );
        vm.serializeAddress(deployed_addresses, "eigenPodBeacon", address(eigenPodBeacon));
        vm.serializeAddress(deployed_addresses, "eigenPodImplementation", address(eigenPodImplementation));
        vm.serializeAddress(deployed_addresses, "baseStrategyImplementation", address(baseStrategyImplementation));
        vm.serializeAddress(deployed_addresses, "emptyContract", address(emptyContract));
        string memory deployed_addresses_output =
            vm.serializeString(deployed_addresses, "strategies", deployed_strategies_output);

        string memory parameters = "parameters";
        vm.serializeAddress(parameters, "executorMultisig", executorMultisig);
        string memory parameters_output = vm.serializeAddress(parameters, "operationsMultisig", operationsMultisig);

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", chainId);

        // serialize all the data
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        vm.serializeString(parent_object, chain_info, chain_info_output);
        string memory finalJson = vm.serializeString(parent_object, parameters, parameters_output);

        console.log("################################################################################");
        console.log("Deployer_M2 writing output");
        console.log("################################################################################");

        // TODO: should output to different file depending on configFile passed to run()
        //       so that we don't override mainnet output by deploying to goerli for eg.
        vm.writeJson(finalJson, "script/output/M2_from_scratch_deployment_data.json");
    }

    function _verifyContractsPointAtOneAnother(
        DelegationManager delegationContract,
        StrategyManager strategyManagerContract,
        Slasher,
        EigenPodManager eigenPodManagerContract,
        RewardsCoordinator rewardsCoordinatorContract
    ) internal view {
        require(delegationContract.slasher() == slasher, "delegation: slasher address not set correctly");
        require(
            delegationContract.strategyManager() == strategyManager,
            "delegation: strategyManager address not set correctly"
        );

        require(strategyManagerContract.slasher() == slasher, "strategyManager: slasher address not set correctly");
        require(
            strategyManagerContract.delegation() == delegation, "strategyManager: delegation address not set correctly"
        );
        require(
            strategyManagerContract.eigenPodManager() == eigenPodManager,
            "strategyManager: eigenPodManager address not set correctly"
        );

        // removing slasher requirements because there is no slasher as part of m2-mainnet release
        // require(slasherContract.strategyManager() == strategyManager, "slasher: strategyManager not set correctly");
        // require(slasherContract.delegation() == delegation, "slasher: delegation not set correctly");

        require(
            eigenPodManagerContract.ethPOS() == ethPOSDeposit,
            " eigenPodManager: ethPOSDeposit contract address not set correctly"
        );
        require(
            eigenPodManagerContract.eigenPodBeacon() == eigenPodBeacon,
            "eigenPodManager: eigenPodBeacon contract address not set correctly"
        );
        require(
            eigenPodManagerContract.strategyManager() == strategyManager,
            "eigenPodManager: strategyManager contract address not set correctly"
        );
        require(
            eigenPodManagerContract.slasher() == slasher, "eigenPodManager: slasher contract address not set correctly"
        );

        require(
            rewardsCoordinatorContract.delegationManager() == delegation,
            "rewardsCoordinator: delegation address not set correctly"
        );

        require(
            rewardsCoordinatorContract.strategyManager() == strategyManager,
            "rewardsCoordinator: strategyManager address not set correctly"
        );
    }

    function _verifyImplementationsSetCorrectly() internal view {
        require(
            eigenLayerProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(delegation))))
                == address(delegationImplementation),
            "delegation: implementation set incorrectly"
        );
        require(
            eigenLayerProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(strategyManager))))
                == address(strategyManagerImplementation),
            "strategyManager: implementation set incorrectly"
        );
        require(
            eigenLayerProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(slasher))))
                == address(slasherImplementation),
            "slasher: implementation set incorrectly"
        );
        require(
            eigenLayerProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(eigenPodManager))))
                == address(eigenPodManagerImplementation),
            "eigenPodManager: implementation set incorrectly"
        );
        require(
            eigenLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(payable(address(rewardsCoordinator)))
            ) == address(rewardsCoordinatorImplementation),
            "rewardsCoordinator: implementation set incorrectly"
        );

        for (uint256 i = 0; i < deployedStrategyArray.length; ++i) {
            require(
                eigenLayerProxyAdmin.getProxyImplementation(
                    TransparentUpgradeableProxy(payable(address(deployedStrategyArray[i])))
                ) == address(baseStrategyImplementation),
                "strategy: implementation set incorrectly"
            );
        }

        require(
            eigenPodBeacon.implementation() == address(eigenPodImplementation),
            "eigenPodBeacon: implementation set incorrectly"
        );
    }

    function _verifyInitialOwners() internal view {
        require(strategyManager.owner() == executorMultisig, "strategyManager: owner not set correctly");
        require(delegation.owner() == executorMultisig, "delegation: owner not set correctly");
        // removing slasher requirements because there is no slasher as part of m2-mainnet release
        // require(slasher.owner() == executorMultisig, "slasher: owner not set correctly");
        require(eigenPodManager.owner() == executorMultisig, "eigenPodManager: owner not set correctly");

        require(eigenLayerProxyAdmin.owner() == executorMultisig, "eigenLayerProxyAdmin: owner not set correctly");
        require(eigenPodBeacon.owner() == executorMultisig, "eigenPodBeacon: owner not set correctly");
    }

    function _checkPauserInitializations() internal view {
        require(delegation.pauserRegistry() == eigenLayerPauserReg, "delegation: pauser registry not set correctly");
        require(
            strategyManager.pauserRegistry() == eigenLayerPauserReg,
            "strategyManager: pauser registry not set correctly"
        );
        // removing slasher requirements because there is no slasher as part of m2-mainnet release
        // require(slasher.pauserRegistry() == eigenLayerPauserReg, "slasher: pauser registry not set correctly");
        require(
            eigenPodManager.pauserRegistry() == eigenLayerPauserReg,
            "eigenPodManager: pauser registry not set correctly"
        );
        require(
            rewardsCoordinator.pauserRegistry() == eigenLayerPauserReg,
            "rewardsCoordinator: pauser registry not set correctly"
        );

        require(eigenLayerPauserReg.isPauser(operationsMultisig), "pauserRegistry: operationsMultisig is not pauser");
        require(eigenLayerPauserReg.isPauser(executorMultisig), "pauserRegistry: executorMultisig is not pauser");
        require(eigenLayerPauserReg.isPauser(pauserMultisig), "pauserRegistry: pauserMultisig is not pauser");
        require(eigenLayerPauserReg.unpauser() == executorMultisig, "pauserRegistry: unpauser not set correctly");

        for (uint256 i = 0; i < deployedStrategyArray.length; ++i) {
            require(
                deployedStrategyArray[i].pauserRegistry() == eigenLayerPauserReg,
                "StrategyBaseTVLLimits: pauser registry not set correctly"
            );
            require(deployedStrategyArray[i].paused() == 0, "StrategyBaseTVLLimits: init paused status set incorrectly");
        }

        // // pause *nothing*
        // uint256 STRATEGY_MANAGER_INIT_PAUSED_STATUS = 0;
        // // pause *everything*
        // uint256 SLASHER_INIT_PAUSED_STATUS = type(uint256).max;
        // // pause *everything*
        // uint256 DELEGATION_INIT_PAUSED_STATUS = type(uint256).max;
        // // pause *all of the proof-related functionality* (everything that can be paused other than creation of EigenPods)
        // uint256 EIGENPOD_MANAGER_INIT_PAUSED_STATUS = (2**1) + (2**2) + (2**3) + (2**4); /* = 30 */
        // // pause *nothing*
        // require(strategyManager.paused() == 0, "strategyManager: init paused status set incorrectly");
        // require(slasher.paused() == type(uint256).max, "slasher: init paused status set incorrectly");
        // require(delegation.paused() == type(uint256).max, "delegation: init paused status set incorrectly");
        // require(eigenPodManager.paused() == 30, "eigenPodManager: init paused status set incorrectly");
    }

    function _verifyInitializationParams() internal view {
        // // one week in blocks -- 50400
        // uint32 STRATEGY_MANAGER_INIT_WITHDRAWAL_DELAY_BLOCKS = 7 days / 12 seconds;
        // require(strategyManager.withdrawalDelayBlocks() == 7 days / 12 seconds,
        //     "strategyManager: withdrawalDelayBlocks initialized incorrectly");
        // uint256 MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR = 32 ether;

        require(
            strategyManager.strategyWhitelister() == operationsMultisig,
            "strategyManager: strategyWhitelister address not set correctly"
        );

        require(
            baseStrategyImplementation.strategyManager() == strategyManager,
            "baseStrategyImplementation: strategyManager set incorrectly"
        );

        require(
            eigenPodImplementation.ethPOS() == ethPOSDeposit,
            "eigenPodImplementation: ethPOSDeposit contract address not set correctly"
        );
        require(
            eigenPodImplementation.eigenPodManager() == eigenPodManager,
            " eigenPodImplementation: eigenPodManager contract address not set correctly"
        );

        string memory config_data = vm.readFile(deployConfigPath);
        for (uint256 i = 0; i < deployedStrategyArray.length; i++) {
            uint256 maxPerDeposit =
                stdJson.readUint(config_data, string.concat(".strategies[", vm.toString(i), "].max_per_deposit"));
            uint256 maxDeposits =
                stdJson.readUint(config_data, string.concat(".strategies[", vm.toString(i), "].max_deposits"));
            (uint256 setMaxPerDeposit, uint256 setMaxDeposits) = deployedStrategyArray[i].getTVLLimits();
            require(setMaxPerDeposit == maxPerDeposit, "setMaxPerDeposit not set correctly");
            require(setMaxDeposits == maxDeposits, "setMaxDeposits not set correctly");
        }
    }
}
