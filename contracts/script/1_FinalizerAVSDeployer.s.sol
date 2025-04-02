// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {BLSSignatureChecker} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {IndexRegistry} from "@eigenlayer-middleware/src/IndexRegistry.sol";
import {IServiceManager} from "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";
import {IRegistryCoordinator, RegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {IStakeRegistry, StakeRegistry} from "@eigenlayer-middleware/src/StakeRegistry.sol";
import {AVSDirectory} from "@eigenlayer/contracts/core/AVSDirectory.sol";
import {DelegationManager} from "@eigenlayer/contracts/core/DelegationManager.sol";
import {RewardsCoordinator} from "@eigenlayer/contracts/core/RewardsCoordinator.sol";
import {StrategyManager} from "@eigenlayer/contracts/core/StrategyManager.sol";
import {PauserRegistry} from "@eigenlayer/contracts/permissions/PauserRegistry.sol";
import {EmptyContract} from "@eigenlayer/test/mocks/EmptyContract.sol";
import {ProxyAdmin, TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Test} from "forge-std/Test.sol";
import {FinalizerServiceManager} from "./../src/FinalizerServiceManager.sol";
import {FinalizerTaskManager, IFinalizerTaskManager} from "./../src/FinalizerTaskManager.sol";
import {OperatorStateRetrieverExtended} from "./../src/OperatorStateRetrieverExtended.sol";
import {Utils} from "./utils/Utils.sol";

// # To deploy and verify our contract
// forge script script/1_FinalizerAvsDeployer.s.sol:Deployer --rpc-url $RPC_URL  --private-key $PRIVATE_KEY --broadcast -vvvv

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Deploys finalizer contracts
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
contract FinalizerAVSDeployer is Script, Test, Utils {
    string internal constant _EIGEN_DEPLOYMENT_PATH = "eigenlayer_deployment_output";
    string internal constant _CONFIG_PATH = "deploy.config";
    string internal constant _OUTPUT_PATH = "avs_deployment_output";

    ProxyAdmin public avsProxyAdmin;
    PauserRegistry public avsPauserReg;
    address public avsOwner;
    address public avsUpgrader;
    bool public allowNonRootTmInit;
    uint32 public taskResponseWindowBlocks;
    // Non-upgradable contracts
    BLSSignatureChecker public blsSignatureChecker;
    OperatorStateRetrieverExtended public operatorStateRetreiverExtended;
    address public dummyRolldown;
    // Upgradeable contracts
    FinalizerServiceManager public serviceManager;
    FinalizerTaskManager public taskManager;
    RegistryCoordinator public registryCoordinator;
    BLSApkRegistry public blsApkRegistry;
    IndexRegistry public indexRegistry;
    StakeRegistry public stakeRegistry;
    // Upgradeable contract implementations
    FinalizerServiceManager public serviceManagerImplementation;
    FinalizerTaskManager public taskManagerImplementation;
    RegistryCoordinator public registryCoordinatorImplementation;
    BLSApkRegistry public blsApkRegistryImplementation;
    IndexRegistry public indexRegistryImplementation;
    StakeRegistry public stakeRegistryImplementation;
    // EigenLayer contracts
    DelegationManager public delegation;
    AVSDirectory public avsDirectory;
    StrategyManager public strategyManager;
    RewardsCoordinator public rewardsCoordinator;

    function run() external {
        // Eigenlayer contracts
        string memory eigenlayerDeployedContracts = readInput(_EIGEN_DEPLOYMENT_PATH);
        strategyManager =
            StrategyManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.strategyManager"));
        delegation = DelegationManager(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.delegationManager"));
        avsDirectory = AVSDirectory(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.avsDirectory"));
        rewardsCoordinator =
            RewardsCoordinator(stdJson.readAddress(eigenlayerDeployedContracts, ".addresses.rewardsCoordinator"));

        string memory configData = readConfig(_CONFIG_PATH);

        // check that the chainID matches the one in the config
        uint256 configChainId = stdJson.readUint(configData, ".chainInfo.chainId");
        emit log_named_uint("You are deploying on ChainID", block.chainid);
        require(configChainId == block.chainid, "You are on the wrong chain for this config");

        address churner = stdJson.readAddress(configData, ".permissions.churner");
        address ejector = stdJson.readAddress(configData, ".permissions.ejector");
        address aggregator = stdJson.readAddress(configData, ".permissions.aggregator");

        avsOwner = stdJson.readAddress(configData, ".permissions.owner");
        avsUpgrader = stdJson.readAddress(configData, ".permissions.upgrader");

        taskResponseWindowBlocks = uint32(stdJson.readUint(configData, ".taskManagerParams.taskResponseWindowBlocks"));

        allowNonRootTmInit = stdJson.readBool(configData, ".allow_non_root_tm_init");

        // START BROADCAST
        vm.startBroadcast();

        // deploy proxy admin for ability to upgrade proxy contracts
        avsProxyAdmin = new ProxyAdmin();

        // deploy pauser registry
        // todo deploy independantly, use owner address for now
        address unpauseMultisig = avsOwner;
        {
            address[] memory pausers = new address[](2);
            pausers[0] = avsOwner;
            pausers[1] = unpauseMultisig;
            avsPauserReg = new PauserRegistry(pausers, unpauseMultisig);
        }

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        EmptyContract emptyContract = new EmptyContract();

        serviceManager = FinalizerServiceManager(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), ""))
        );
        taskManager = FinalizerTaskManager(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), ""))
        );
        registryCoordinator = RegistryCoordinator(
            address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), ""))
        );
        blsApkRegistry =
            BLSApkRegistry(address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), "")));
        indexRegistry =
            IndexRegistry(address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), "")));
        stakeRegistry =
            StakeRegistry(address(new TransparentUpgradeableProxy(address(emptyContract), address(avsProxyAdmin), "")));

        // deploy StakeRegistry
        stakeRegistryImplementation = new StakeRegistry(registryCoordinator, delegation);

        // upgrade stake registry proxy to implementation
        avsProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(stakeRegistry))), address(stakeRegistryImplementation)
        );

        // deploy BLSApkRegistry
        blsApkRegistryImplementation = new BLSApkRegistry(registryCoordinator);

        // upgrade bls pubkey registry proxy to implementation
        avsProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(blsApkRegistry))), address(blsApkRegistryImplementation)
        );

        //deploy IndexRegistry
        indexRegistryImplementation = new IndexRegistry(registryCoordinator);

        // upgrade index registry proxy to implementation
        avsProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(indexRegistry))), address(indexRegistryImplementation)
        );

        //deploy serviceManager
        {
            uint64 recurrentRegistrationLimit =
                uint64(stdJson.readUint(configData, ".registryParams.recurrentRegistrationLimit"));

            serviceManagerImplementation = new FinalizerServiceManager(
                avsDirectory,
                rewardsCoordinator,
                registryCoordinator,
                stakeRegistry,
                IFinalizerTaskManager(address(taskManager)),
                recurrentRegistrationLimit
            );
        }

        // upgrade service manager proxy to implementation and initialize
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(serviceManager))),
            address(serviceManagerImplementation),
            abi.encodeWithSelector(serviceManager.initialize.selector, avsOwner, avsOwner, ejector)
        );

        // deploy RegistryCoordinator
        registryCoordinatorImplementation = new RegistryCoordinator(
            IServiceManager(address(serviceManager)), stakeRegistry, blsApkRegistry, indexRegistry
        );

        (
            uint96[] memory minimumStakeForQuorum,
            IStakeRegistry.StrategyParams[][] memory strategyAndWeightingMultipliers
        ) = _parseStakeRegistryParams(configData);

        // parse initalization params and permissions from config data
        bytes memory operatorConfigsRaw = stdJson.parseRaw(configData, ".operatorSetParams");
        IRegistryCoordinator.OperatorSetParam[] memory operatorSetParams =
            abi.decode(operatorConfigsRaw, (IRegistryCoordinator.OperatorSetParam[]));

        // upgrade registry coordinator proxy to implementation and initialize
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(registryCoordinator))),
            address(registryCoordinatorImplementation),
            abi.encodeWithSelector(
                RegistryCoordinator.initialize.selector,
                avsOwner,
                churner,
                address(serviceManager),
                avsPauserReg,
                0,
                operatorSetParams,
                minimumStakeForQuorum,
                strategyAndWeightingMultipliers
            )
        );

        blsSignatureChecker = new BLSSignatureChecker(registryCoordinator);
        // This is a hack to set BlsSignatureChecker's staleStakesForbidden flag
        // We do it this way to avoid forking it...
        // This hack depends on avsOwner being the same as the deployer...
        blsSignatureChecker.setStaleStakesForbidden(false);

        operatorStateRetreiverExtended = new OperatorStateRetrieverExtended();

        taskManagerImplementation = new FinalizerTaskManager();

        // upgrade task manager proxy to implementation and initialize
        avsProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(payable(address(taskManager))),
            address(taskManagerImplementation),
            abi.encodeWithSelector(
                taskManager.initialize.selector,
                avsPauserReg,
                avsOwner,
                aggregator,
                aggregator,
                allowNonRootTmInit,
                blsSignatureChecker,
                taskResponseWindowBlocks,
                operatorStateRetreiverExtended,
                dummyRolldown
            )
        );

        // transfer ownership of proxy admin to upgrader
        avsProxyAdmin.transferOwnership(avsUpgrader);

        // end deployment
        vm.stopBroadcast();

        // sanity checks
        _verifyContractPointers(serviceManager, registryCoordinator, blsApkRegistry, indexRegistry, stakeRegistry);

        _verifyContractPointers(
            serviceManagerImplementation,
            registryCoordinatorImplementation,
            blsApkRegistryImplementation,
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

    function _parseStakeRegistryParams(string memory configData)
        internal
        pure
        returns (
            uint96[] memory minimumStakeForQuorum,
            IStakeRegistry.StrategyParams[][] memory strategyAndWeightingMultipliers
        )
    {
        bytes memory stakesConfigsRaw = stdJson.parseRaw(configData, ".minimumStakes");
        minimumStakeForQuorum = abi.decode(stakesConfigsRaw, (uint96[]));

        bytes memory strategyConfigsRaw = stdJson.parseRaw(configData, ".strategyWeights");
        strategyAndWeightingMultipliers = abi.decode(strategyConfigsRaw, (IStakeRegistry.StrategyParams[][]));
    }

    function _verifyContractPointers(
        FinalizerServiceManager _serviceManager,
        RegistryCoordinator _registryCoordinator,
        BLSApkRegistry _blsApkRegistry,
        IndexRegistry _indexRegistry,
        StakeRegistry _stakeRegistry
    ) internal view {
        require(_serviceManager.taskManager() == taskManager, "serviceManager.taskManager() != taskManager");

        require(
            address(_registryCoordinator.serviceManager()) == address(serviceManager),
            "registryCoordinator.serviceManager() != serviceManager"
        );
        require(
            _registryCoordinator.stakeRegistry() == stakeRegistry,
            "registryCoordinator.stakeRegistry() != stakeRegistry"
        );
        require(
            _registryCoordinator.blsApkRegistry() == blsApkRegistry,
            "registryCoordinator.blsApkRegistry() != blsApkRegistry"
        );
        require(
            _registryCoordinator.indexRegistry() == indexRegistry,
            "registryCoordinator.indexRegistry() != indexRegistry"
        );

        require(
            _blsApkRegistry.registryCoordinator() == address(registryCoordinator),
            "blsApkRegistry.registryCoordinator() != registryCoordinator"
        );

        require(
            _indexRegistry.registryCoordinator() == address(registryCoordinator),
            "indexRegistry.registryCoordinator() != registryCoordinator"
        );

        require(
            _stakeRegistry.registryCoordinator() == address(registryCoordinator),
            "stakeRegistry.registryCoordinator() != registryCoordinator"
        );
        require(_stakeRegistry.delegation() == delegation, "stakeRegistry.delegation() != delegation");
    }

    function _verifyImplementations() internal view {
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(serviceManager))))
                == address(serviceManagerImplementation),
            "serviceManager: implementation set incorrectly"
        );
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(registryCoordinator))))
                == address(registryCoordinatorImplementation),
            "registryCoordinator: implementation set incorrectly"
        );
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(blsApkRegistry))))
                == address(blsApkRegistryImplementation),
            "blsApkRegistry: implementation set incorrectly"
        );
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(indexRegistry))))
                == address(indexRegistryImplementation),
            "indexRegistry: implementation set incorrectly"
        );
        require(
            avsProxyAdmin.getProxyImplementation(TransparentUpgradeableProxy(payable(address(stakeRegistry))))
                == address(stakeRegistryImplementation),
            "stakeRegistry: implementation set incorrectly"
        );
    }

    function _verifyInitalizations(
        address churner,
        address ejector,
        IRegistryCoordinator.OperatorSetParam[] memory operatorSetParams,
        uint96[] memory minimumStakeForQuorum,
        IStakeRegistry.StrategyParams[][] memory strategyAndWeightingMultipliers
    ) internal view {
        require(serviceManager.owner() == avsOwner, "serviceManager.owner() != avsOwner");
        require(serviceManager.ejector() == ejector, "serviceManager.ejector() != ejector");

        require(registryCoordinator.churnApprover() == churner, "registryCoordinator.churner() != churner");
        require(
            registryCoordinator.ejector() == address(serviceManager), "registryCoordinator.ejector() != serviceManager"
        );
        require(
            registryCoordinator.pauserRegistry() == avsPauserReg,
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

        for (uint8 i = 0; i < minimumStakeForQuorum.length; ++i) {
            require(
                stakeRegistry.minimumStakeForQuorum(i) == minimumStakeForQuorum[i],
                "stakeRegistry.minimumStakeForQuorum != minimumStakeForQuorum"
            );
        }

        for (uint8 i = 0; i < strategyAndWeightingMultipliers.length; ++i) {
            for (uint8 j = 0; j < strategyAndWeightingMultipliers[i].length; ++j) {
                IStakeRegistry.StrategyParams memory params = stakeRegistry.strategyParamsByIndex(i, j);
                require(
                    address(params.strategy) == address(strategyAndWeightingMultipliers[i][j].strategy),
                    "stakeRegistry.strategyAndWeightingMultipliers != strategyAndWeightingMultipliers"
                );
                require(
                    params.multiplier == strategyAndWeightingMultipliers[i][j].multiplier,
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
        vm.serializeAddress(deployed_addresses, "avsProxyAdmin", address(avsProxyAdmin));
        vm.serializeAddress(deployed_addresses, "avsPauseReg", address(avsPauserReg));
        vm.serializeAddress(deployed_addresses, "serviceManager", address(serviceManager));
        vm.serializeAddress(deployed_addresses, "serviceManagerImplementation", address(serviceManagerImplementation));
        vm.serializeAddress(deployed_addresses, "taskManager", address(taskManager));
        vm.serializeAddress(deployed_addresses, "taksManagerImplementation", address(taskManagerImplementation));
        vm.serializeAddress(deployed_addresses, "registryCoordinator", address(registryCoordinator));
        vm.serializeAddress(
            deployed_addresses, "registryCoordinatorImplementation", address(registryCoordinatorImplementation)
        );
        vm.serializeAddress(deployed_addresses, "blsApkRegistry", address(blsApkRegistry));
        vm.serializeAddress(deployed_addresses, "blsApkRegistryImplementation", address(blsApkRegistryImplementation));
        vm.serializeAddress(deployed_addresses, "indexRegistry", address(indexRegistry));
        vm.serializeAddress(deployed_addresses, "indexRegistryImplementation", address(indexRegistryImplementation));
        vm.serializeAddress(deployed_addresses, "stakeRegistry", address(stakeRegistry));
        string memory deployed_addresses_output =
            vm.serializeAddress(deployed_addresses, "stakeRegistryImplementation", address(stakeRegistryImplementation));

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(chain_info, "chainId", block.chainid);

        string memory permissions = "permissions";
        vm.serializeAddress(permissions, "avsOwner", avsOwner);
        vm.serializeAddress(permissions, "avsUpgrader", avsUpgrader);
        vm.serializeAddress(permissions, "avsChurner", churner);
        vm.serializeAddress(permissions, "avsEjector", ejector);
        vm.serializeAddress(permissions, "avsMultisig", multisig);
        string memory permissions_output = vm.serializeAddress(permissions, "aggregator", aggregator);

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(parent_object, deployed_addresses, deployed_addresses_output);
        string memory finalJson = vm.serializeString(parent_object, permissions, permissions_output);
        console.logString(finalJson);
        writeOutput(finalJson, _OUTPUT_PATH);
    }

    function getOutputPath() external pure returns (string memory) {
        return _OUTPUT_PATH;
    }
}
