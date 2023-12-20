// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import "@eigenlayer/contracts/core/StrategyManager.sol";
import "@eigenlayer/contracts/core/Slasher.sol";
import "@eigenlayer/contracts/core/DelegationManager.sol";
import "@eigenlayer/test/mocks/EmptyContract.sol";

import "@eigenlayer-middleware/src/BLSPublicKeyCompendium.sol";
import "@eigenlayer-middleware/src/BLSRegistryCoordinatorWithIndices.sol";
import "@eigenlayer-middleware/src/BLSPubkeyRegistry.sol";
import "@eigenlayer-middleware/src/IndexRegistry.sol";
import "@eigenlayer-middleware/src/StakeRegistry.sol";
import "@eigenlayer-middleware/src/BLSOperatorStateRetriever.sol";

import {MangataServiceManager, IServiceManager} from "../src/MangataServiceManager.sol";
import {MangataTaskManager} from "../src/MangataTaskManager.sol";
import {IMangataTaskManager} from "../src/IMangataTaskManager.sol";

import {Utils} from "./utils/Utils.sol";

import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console.sol";

// # To deploy and verify our contract
// forge script script/1_MangataAvsDeployer.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv
contract Deployer is Script, Utils, Test {
    string constant EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string constant SHARED_AVS_DEPLOYMENT_PATH = "shared_avs_contracts_deployment_output";
    string constant CONFIG_PATH = "deploy.config";
    string constant OUTPUT_PATH = "mangata_avs_deployment_output_";

    ProxyAdmin public mangataProxyAdmin;
    PauserRegistry public mangataPauserReg;
    address public mangataOwner;
    address public mangataUpgrader;

    //upgradeable contracts
    MangataServiceManager public serviceManager;
    MangataTaskManager public taskManager;
    BLSRegistryCoordinatorWithIndices public registryCoordinator;
    BLSPubkeyRegistry public blsPubkeyRegistry;
    IndexRegistry public indexRegistry;
    StakeRegistry public stakeRegistry;

    //upgradeable contract implementations
    MangataServiceManager public serviceManagerImplementation;
    MangataTaskManager public taskManagerImplementation;
    BLSRegistryCoordinatorWithIndices public registryCoordinatorImplementation;
    BLSPubkeyRegistry public blsPubkeyRegistryImplementation;
    IndexRegistry public indexRegistryImplementation;
    StakeRegistry public stakeRegistryImplementation;

    // EigenLayer Contracts
    Slasher public slasher;
    DelegationManager public delegation;
    StrategyManager public strategyManager;
    BLSPublicKeyCompendium public pubkeyCompendium;
    BLSOperatorStateRetriever public blsOperatorStateRetriever;

    function run() external {
        // Eigenlayer contracts
        string memory eigenlayerDeployedContracts = readInput(EIGEN_DEPLOYMENT_PATH);
        strategyManager =
            StrategyManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.strategyManager"));
        slasher = Slasher(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.slasher"));
        delegation = DelegationManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.delegation"));

        string memory sharedAvsDeployedContracts = readInput(SHARED_AVS_DEPLOYMENT_PATH);
        pubkeyCompendium =
            BLSPublicKeyCompendium(stdJson.readAddress(sharedAvsDeployedContracts, ".blsPublicKeyCompendium"));
        blsOperatorStateRetriever =
            BLSOperatorStateRetriever(stdJson.readAddress(sharedAvsDeployedContracts, ".blsOperatorStateRetriever"));

        // READ JSON CONFIG DATA
        string memory configData = readConfig(CONFIG_PATH);

        // check that the chainID matches the one in the config
        uint256 currentChainId = block.chainid;
        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        emit log_named_uint("You are deploying on ChainID", currentChainId);
        require(configChainId == currentChainId, "You are on the wrong chain for this config");

        // parse initalization params and permissions from config data
        bytes memory operatorConfigsRaw = stdJson.parseRaw(configData, ".operatorSetParams");
        IBLSRegistryCoordinatorWithIndices.OperatorSetParam[] memory operatorSetParams =
            abi.decode(operatorConfigsRaw, (IBLSRegistryCoordinatorWithIndices.OperatorSetParam[]));

        address churner = stdJson.readAddress(configData, ".permissions.churner");
        address ejector = stdJson.readAddress(configData, ".permissions.ejector");
        address aggregator = stdJson.readAddress(configData, ".permissions.aggregator");

        mangataOwner = stdJson.readAddress(configData, ".permissions.owner");
        mangataUpgrader = stdJson.readAddress(configData, ".permissions.upgrader");

        uint32 taskResponseWindowBlocks = uint32(stdJson.readUint(configData, ".taskManagerParams.taskResponseWindowBlocks"));

        (
            uint96[] memory minimumStakeForQuorum,
            IVoteWeigher.StrategyAndWeightingMultiplier[][] memory strategyAndWeightingMultipliers
        ) = _parseStakeRegistryParams(configData);

        // START BROADCAST
        vm.startBroadcast();

        // deploy proxy admin for ability to upgrade proxy contracts
        mangataProxyAdmin = new ProxyAdmin();

        // deploy pauser registry
        // todo deploy independantly, use owner address for now
        address unpauseMultisig = mangataOwner;
        {
            address[] memory pausers = new address[](2);
            pausers[0] = mangataOwner;
            pausers[1] = unpauseMultisig;
            mangataPauserReg = new PauserRegistry(
                pausers,
                unpauseMultisig
            );
        }

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();

        serviceManager = MangataServiceManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );
        taskManager = MangataTaskManager(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );
        registryCoordinator = BLSRegistryCoordinatorWithIndices(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );
        blsPubkeyRegistry = BLSPubkeyRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );
        indexRegistry = IndexRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );
        stakeRegistry = StakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(mangataProxyAdmin),
                    ""
                )
            )
        );

        // deploy StakeRegistry
        stakeRegistryImplementation = new StakeRegistry(
            registryCoordinator,
            strategyManager,
            IServiceManager(address(serviceManager))
        );

        // upgrade stake registry proxy to implementation and initialize
        mangataProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(stakeRegistry))),
            address(stakeRegistryImplementation),
            abi.encodeWithSelector(
                StakeRegistry.initialize.selector, minimumStakeForQuorum, strategyAndWeightingMultipliers
            )
        );

        // deploy RegistryCoordinator
        registryCoordinatorImplementation = new BLSRegistryCoordinatorWithIndices(
            slasher,
            IServiceManager(address(serviceManager)),
            stakeRegistry,
            blsPubkeyRegistry,
            indexRegistry
        );

        // upgrade registry coordinator proxy to implementation and initialize
        mangataProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(registryCoordinator))),
            address(registryCoordinatorImplementation),
            abi.encodeWithSelector(
                BLSRegistryCoordinatorWithIndices.initialize.selector,
                churner,
                ejector,
                operatorSetParams,
                mangataPauserReg,
                0
            )
        );

        // deploy BLSPubkeyRegistry
        blsPubkeyRegistryImplementation = new BLSPubkeyRegistry(
            registryCoordinator,
            pubkeyCompendium
        );

        // upgrade bls pubkey registry proxy to implementation
        mangataProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(blsPubkeyRegistry))), address(blsPubkeyRegistryImplementation)
        );

        //deploy IndexRegistry
        indexRegistryImplementation = new IndexRegistry(registryCoordinator);

        // upgrade index registry proxy to implementation
        mangataProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(indexRegistry))), address(indexRegistryImplementation)
        );

        //deploy serviceManager
        serviceManagerImplementation = new MangataServiceManager(
            registryCoordinator,
            slasher,
            IMangataTaskManager(address(taskManager))
        );

        // upgrade service manager proxy to implementation and initialize
        mangataProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(serviceManager))),
            address(serviceManagerImplementation),
            abi.encodeWithSelector(serviceManager.initialize.selector, mangataPauserReg, mangataOwner)
        );

        taskManagerImplementation = new MangataTaskManager(
            registryCoordinator,
            taskResponseWindowBlocks
        );

        // upgrade task manager proxy to implementation and initialize
        mangataProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(taskManager))),
            address(taskManagerImplementation),
            abi.encodeWithSelector(
                taskManager.initialize.selector, mangataPauserReg, mangataOwner, aggregator, aggregator
            )
        );

        // transfer ownership of proxy admin to upgrader
        mangataProxyAdmin.transferOwnership(mangataUpgrader);

        // end deployment
        vm.stopBroadcast();

        // sanity checks
        _verifyContractPointers(serviceManager, registryCoordinator, blsPubkeyRegistry, indexRegistry, stakeRegistry);

        _verifyContractPointers(
            serviceManagerImplementation,
            registryCoordinatorImplementation,
            blsPubkeyRegistryImplementation,
            indexRegistryImplementation,
            stakeRegistryImplementation
        );

        _verifyImplementations();
        _verifyInitalizations(
            churner, ejector, operatorSetParams, minimumStakeForQuorum, strategyAndWeightingMultipliers
        );

        //write output
        _writeOutput(churner, ejector, aggregator, unpauseMultisig);
    }

    function _parseStakeRegistryParams(string memory config_data)
        internal
        pure
        returns (
            uint96[] memory minimumStakeForQuorum,
            IVoteWeigher.StrategyAndWeightingMultiplier[][] memory strategyAndWeightingMultipliers
        )
    {
        bytes memory stakesConfigsRaw = stdJson.parseRaw(config_data, ".minimumStakes");
        minimumStakeForQuorum = abi.decode(stakesConfigsRaw, (uint96[]));

        bytes memory strategyConfigsRaw = stdJson.parseRaw(config_data, ".strategyWeights");
        strategyAndWeightingMultipliers =
            abi.decode(strategyConfigsRaw, (IVoteWeigher.StrategyAndWeightingMultiplier[][]));
    }

    function _verifyContractPointers(
        MangataServiceManager _serviceManager,
        BLSRegistryCoordinatorWithIndices _registryCoordinator,
        BLSPubkeyRegistry _blsPubkeyRegistry,
        IndexRegistry _indexRegistry,
        StakeRegistry _stakeRegistry
    ) internal view {
        require(
            _serviceManager.registryCoordinator() == registryCoordinator,
            "serviceManager.registryCoordinator() != registryCoordinator"
        );
        require(_serviceManager.taskManager() == taskManager, "serviceManager.taskManager() != taskManager");
        require(_serviceManager.slasher() == slasher, "serviceManager.slasher() != slasher");

        require(_registryCoordinator.slasher() == slasher, "registryCoordinator.slasher() != slasher");
        require(
            address(_registryCoordinator.serviceManager()) == address(serviceManager),
            "registryCoordinator.serviceManager() != serviceManager"
        );
        require(
            _registryCoordinator.stakeRegistry() == stakeRegistry,
            "registryCoordinator.stakeRegistry() != stakeRegistry"
        );
        require(
            _registryCoordinator.blsPubkeyRegistry() == blsPubkeyRegistry,
            "registryCoordinator.blsPubkeyRegistry() != blsPubkeyRegistry"
        );
        require(
            _registryCoordinator.indexRegistry() == indexRegistry,
            "registryCoordinator.indexRegistry() != indexRegistry"
        );

        require(
            _blsPubkeyRegistry.registryCoordinator() == registryCoordinator,
            "blsPubkeyRegistry.registryCoordinator() != registryCoordinator"
        );
        require(
            _blsPubkeyRegistry.pubkeyCompendium() == pubkeyCompendium,
            "blsPubkeyRegistry.pubkeyCompendium() != pubkeyCompendium"
        );

        require(
            _indexRegistry.registryCoordinator() == registryCoordinator,
            "indexRegistry.registryCoordinator() != registryCoordinator"
        );

        require(
            _stakeRegistry.registryCoordinator() == registryCoordinator,
            "stakeRegistry.registryCoordinator() != registryCoordinator"
        );
        require(
            _stakeRegistry.strategyManager() == strategyManager, "stakeRegistry.strategyManager() != strategyManager"
        );
        require(
            address(_stakeRegistry.serviceManager()) == address(serviceManager),
            "stakeRegistry.serviceManager() != serviceManager"
        );
    }

    function _verifyImplementations() internal view {
        require(
            mangataProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(serviceManager))))
                == address(serviceManagerImplementation),
            "serviceManager: implementation set incorrectly"
        );
        require(
            mangataProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(registryCoordinator))))
                == address(registryCoordinatorImplementation),
            "registryCoordinator: implementation set incorrectly"
        );
        require(
            mangataProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(blsPubkeyRegistry))))
                == address(blsPubkeyRegistryImplementation),
            "blsPubkeyRegistry: implementation set incorrectly"
        );
        require(
            mangataProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(indexRegistry))))
                == address(indexRegistryImplementation),
            "indexRegistry: implementation set incorrectly"
        );
        require(
            mangataProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(stakeRegistry))))
                == address(stakeRegistryImplementation),
            "stakeRegistry: implementation set incorrectly"
        );
    }

    function _verifyInitalizations(
        address churner,
        address ejector,
        IBLSRegistryCoordinatorWithIndices.OperatorSetParam[] memory operatorSetParams,
        uint96[] memory minimumStakeForQuorum,
        IVoteWeigher.StrategyAndWeightingMultiplier[][] memory strategyAndWeightingMultipliers
    ) internal view {
        require(serviceManager.owner() == mangataOwner, "serviceManager.owner() != mangataOwner");
        require(
            serviceManager.pauserRegistry() == mangataPauserReg, "serviceManager: pauser registry not set correctly"
        );
        require(strategyManager.paused() == 0, "serviceManager: init paused status set incorrectly");

        require(registryCoordinator.churnApprover() == churner, "registryCoordinator.churner() != churner");
        require(registryCoordinator.ejector() == ejector, "registryCoordinator.ejector() != ejector");
        require(
            registryCoordinator.pauserRegistry() == mangataPauserReg,
            "registryCoordinator: pauser registry not set correctly"
        );
        require(registryCoordinator.paused() == 0, "registryCoordinator: init paused status set incorrectly");

        for (uint8 i = 0; i < operatorSetParams.length; ++i) {
            require(
                keccak256(abi.encode(registryCoordinator.getOperatorSetParams(i)))
                    == keccak256(abi.encode(operatorSetParams[i])),
                "registryCoordinator.operatorSetParams != operatorSetParams"
            );
        }

        for (uint256 i = 0; i < minimumStakeForQuorum.length; ++i) {
            require(
                stakeRegistry.minimumStakeForQuorum(i) == minimumStakeForQuorum[i],
                "stakeRegistry.minimumStakeForQuorum != minimumStakeForQuorum"
            );
        }

        for (uint8 i = 0; i < strategyAndWeightingMultipliers.length; ++i) {
            for (uint8 j = 0; j < strategyAndWeightingMultipliers[i].length; ++j) {
                (IStrategy strategy, uint96 multiplier) = stakeRegistry.strategiesConsideredAndMultipliers(i, j);
                require(
                    address(strategy) == address(strategyAndWeightingMultipliers[i][j].strategy),
                    "stakeRegistry.strategyAndWeightingMultipliers != strategyAndWeightingMultipliers"
                );
                require(
                    multiplier == strategyAndWeightingMultipliers[i][j].multiplier,
                    "stakeRegistry.strategyAndWeightingMultipliers != strategyAndWeightingMultipliers"
                );
            }
        }

        require(
            operatorSetParams.length == strategyAndWeightingMultipliers.length
                && operatorSetParams.length == minimumStakeForQuorum.length,
            "operatorSetParams, strategyAndWeightingMultipliers, and minimumStakeForQuorum must be the same length"
        );
    }

    function _writeOutput(address churner, address ejector, address aggregator, address multisig) internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(deployed_addresses, "mangataProxyAdmin", address(mangataProxyAdmin));
        vm.serializeAddress(deployed_addresses, "mangataPauseReg", address(mangataPauserReg));
        vm.serializeAddress(deployed_addresses, "blsPubKeyCompendium", address(pubkeyCompendium));
        vm.serializeAddress(deployed_addresses, "blsOperatorStateRetriever", address(blsOperatorStateRetriever));
        vm.serializeAddress(deployed_addresses, "slasher", address(slasher));
        vm.serializeAddress(deployed_addresses, "serviceManager", address(serviceManager));
        vm.serializeAddress(deployed_addresses, "serviceManagerImplementation", address(serviceManagerImplementation));
        vm.serializeAddress(deployed_addresses, "taskManager", address(taskManager));
        vm.serializeAddress(deployed_addresses, "taksManagerImplementation", address(taskManagerImplementation));
        vm.serializeAddress(deployed_addresses, "registryCoordinator", address(registryCoordinator));
        vm.serializeAddress(
            deployed_addresses, "registryCoordinatorImplementation", address(registryCoordinatorImplementation)
        );
        vm.serializeAddress(deployed_addresses, "blsPubkeyRegistry", address(blsPubkeyRegistry));
        vm.serializeAddress(
            deployed_addresses, "blsPubkeyRegistryImplementation", address(blsPubkeyRegistryImplementation)
        );
        vm.serializeAddress(deployed_addresses, "indexRegistry", address(indexRegistry));
        vm.serializeAddress(deployed_addresses, "indexRegistryImplementation", address(indexRegistryImplementation));
        vm.serializeAddress(deployed_addresses, "stakeRegistry", address(stakeRegistry));
        string memory deployed_addresses_output =
            vm.serializeAddress(deployed_addresses, "stakeRegistryImplementation", address(stakeRegistryImplementation));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "mangataOwner", mangataOwner);
        vm.serializeAddress(permissions, "mangataUpgrader", mangataUpgrader);
        vm.serializeAddress(permissions, "mangataChurner", churner);
        vm.serializeAddress(permissions, "mangataEjector", ejector);
        vm.serializeAddress(permissions, "mangataMultisig", multisig);
        string memory permissions_output = vm.serializeAddress(permissions, "aggregator", aggregator);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        writeOutput(finalJson, string.concat(OUTPUT_PATH, stdJson.readString(finalJson, ".chainInfo.deploymentBlock")));
    }
}
