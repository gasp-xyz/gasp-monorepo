import { useEffect, useState } from "react";
import "./App.css";
import { ethers } from "ethers";
import gaspLogo from "./images/gasp.logo.svg"
import {gaspFaucetContract, gethFaucetContract} from "./faucet";

function App() {
    const [walletAddress, setWalletAddress] = useState("");
    const [signer, setSigner] = useState();
    const [gaspFcContract, setGaspFcContract] = useState();
    const [gethFcContract, setGethFcContract] = useState();
    const [chainId, setChainId] = useState();
    const [withdrawError, setWithdrawError] = useState("");
    const [withdrawSuccess, setWithdrawSuccess] = useState("");
    const [contractGaspBalance, setContractGaspBalance] = useState(0);
    const [contractGethBalance, setContractGethBalance] = useState(0);

    useEffect(() => {
        getCurrentWalletConnected();
        addWalletListener();
        getGaspContractBalance()
        getGethContractBalance()
    }, [walletAddress]);

    useEffect(() => {
        const getCurrentChainId = async () => {
            if(window.ethereum) {
                const provider = new ethers.providers.Web3Provider(window.ethereum);
                const metamaskChainId = await provider.send("eth_chainId");
                setChainId(metamaskChainId);
            }
        }

        getCurrentChainId()
    },[chainId])

    useEffect(() => {
        if(window.ethereum) {
            window.ethereum.on('chainChanged', () => {
                window.location.reload();
            })
        }
    });

    const handleChangeWallet = (event) => {
        setWalletAddress(event.target.value);
    };

    const getGaspContractBalance = async () => {
        if (gaspFcContract) {
            try {
                const balance = await gaspFcContract.getBalance()
                setContractGaspBalance(parseInt(ethers.utils.formatEther(balance)))
            } catch (err) {
                console.log(err.message);
            }
        }
    };

    const getGethContractBalance = async () => {
        if (gethFcContract) {
            try {
                const balance = await gethFcContract.getBalance()
                setContractGethBalance(parseInt(ethers.utils.formatEther(balance)))
            } catch (err) {
                console.log(err.message);
            }
        }
    };

    const connectWallet = async () => {
        if (typeof window != "undefined" && typeof window.ethereum != "undefined") {
            try {
                /* get provider */
                const provider = new ethers.providers.Web3Provider(window.ethereum);
                /* get accounts */
                const accounts = await provider.send("eth_requestAccounts", []);
                /* get signer */
                setSigner(provider.getSigner());
                /* local contract instance */
                setGaspFcContract(gaspFaucetContract(provider));
                setGethFcContract(gethFaucetContract(provider));

                /* set active wallet address */
                setWalletAddress(accounts[0]);
            } catch (err) {
                console.error(err.message);
            }
        } else {
            /* MetaMask is not installed */
            console.log("Please install MetaMask");
        }
    };

    const getCurrentWalletConnected = async () => {
        if (typeof window != "undefined" && typeof window.ethereum != "undefined") {
            try {
                /* get provider */
                const provider = new ethers.providers.Web3Provider(window.ethereum);
                /* get accounts */
                const accounts = await provider.send("eth_accounts", []);
                if (accounts.length > 0) {
                    /* get signer */
                    setSigner(provider.getSigner());
                    /* local contract instance */
                    setGaspFcContract(gaspFaucetContract(provider));
                    setGethFcContract(gethFaucetContract(provider));
                    /* set active wallet address */
                    setWalletAddress(accounts[0]);
                } else {
                    console.log("Connect to MetaMask using the Connect Wallet button");
                }
            } catch (err) {
                console.error(err.message);
            }
        } else {
            /* MetaMask is not installed */
            console.log("Please install MetaMask");
        }
    };

    const addWalletListener = async () => {
        if (typeof window != "undefined" && typeof window.ethereum != "undefined") {
            window.ethereum.on("accountsChanged", (accounts) => {
                setWalletAddress(accounts[0]);
            });
        } else {
            /* MetaMask is not installed */
            setWalletAddress("");
            console.log("Please install MetaMask");
        }
    };

    const getGASPHandler = async () => {
        setWithdrawError("");
        setWithdrawSuccess("");
        try {
            const fcContractWithSigner = gaspFcContract.connect(signer);
            await fcContractWithSigner.requestTokens();
            setWithdrawSuccess("Operation succeeded - enjoy your tokens!");
        } catch (err) {
            setWithdrawError(JSON.stringify(err.reason));
        }
    };

    const getGETHHandler = async () => {
        setWithdrawError("");
        setWithdrawSuccess("");
        try {
            const fcContractWithSigner = gethFcContract.connect(signer);
            await fcContractWithSigner.requestTokens();
            setWithdrawSuccess("Operation succeeded - enjoy your tokens!");
        } catch (err) {
            setWithdrawError(JSON.stringify(err.reason));
        }
    };

    const addGaspToMetamsk = async () => {
        try {
            await window.ethereum.request({
                "method": "wallet_watchAsset",
                "params": {
                    "type": "ERC20",
                    "options": {
                        "address": "0x1317106Dd45FF0EB911e9F0aF78D63FBF9076f69",
                        "symbol": "GASP",
                        "decimals": 18
                    }
                }
            });
        } catch (e) {
            setWithdrawError(JSON.stringify(e.message));
        }
    };

    const addGethToMetamsk = async () => {
        try {
            await window.ethereum.request({
                "method": "wallet_watchAsset",
                "params": {
                    "type": "ERC20",
                    "options": {
                        "address": "0xce658E386fAc0646CEDAf810070f0525Af14D11d",
                        "symbol": "GETH",
                        "decimals": 18
                    }
                }
            });
        } catch (e) {
            setWithdrawError(JSON.stringify(e.message));
        }
    };

    const addHoleskyNetwork = async () => {
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        try {
            await provider.send("wallet_switchEthereumChain", [{ chainId: "0x4268" }]);
        } catch (switchError) {
            // This error code indicates that the chain has not been added to MetaMask.
            if (switchError.code === 4902) {
                try {
                    await provider.send("wallet_addEthereumChain", [{
                        chainId: "0x4268",
                        chainName: "Holesky",
                        rpcUrls: ["https://rpc.holesky.ethpandaops.io"] /* ... */,
                    }])
                } catch (addError) {
                    console.error(addError)
                }
            }
            console.error(switchError)
        }
    };

    return (
        <section className="hero is-fullheight">
            <div className="hero-head">
                <header className="navbar">
                    <div className="container">
                        <div className="navbar-brand">
                            <a className="navbar-item">
                                <img style={{transform: 'scale(1.3)'}} src={gaspLogo} alt="Logo"/>
                            </a>
                            <span className="navbar-burger" data-target="navbarMenuHeroC">
               <span></span>
               <span></span>
               <span></span>
               <span></span>
               </span>
                        </div>
                        <div id="navbarMenuHeroC" className="navbar-menu">
                            <div className="navbar-end">
                  <span className="navbar-item">
                  <a className="button is-inverted" onClick={connectWallet}>
                  <span className="has-text-weight-bold">
                  {walletAddress && walletAddress.length > 0
                      ? <span>{`Connected: ${walletAddress.substring(
                          0,
                          6
                      )}...${walletAddress.substring(38)}`}</span>
                      : <span>Connect Metamask</span>}
                  </span>
                  </a>
                  </span>
                            </div>
                        </div>
                    </div>
                </header>
                <nav className="level is-mobile">
                    <div className="level-item has-text-centered">
                        <div>
                            <p className="heading has-text-black">GASP Tokens</p>
                            <p className="title has-text-black">10</p>
                        </div>
                    </div>
                    <div className="level-item has-text-centered">
                        <div>
                            <p className="heading has-text-black">GETH Tokens</p>
                            <p className="title has-text-black">10</p>
                        </div>
                    </div>
                    <div className="level-item has-text-centered">
                        <div>
                            <p className="heading has-text-black">GASP Contract Tokens</p>
                            <p className="title has-text-black">{contractGaspBalance}</p>
                        </div>
                    </div>
                    <div className="level-item has-text-centered">
                        <div>
                            <p className="heading has-text-black">GETH Contract Tokens</p>
                            <p className="title has-text-black">{contractGethBalance}</p>
                        </div>
                    </div>
                </nav>
            </div>
            <div className="container mt-5">
                {chainId === "0x4268" ? (
                    <span className="tag is-success is-size-3">Connected to Holesky</span>
                ) : (
                    <span className="tag is-danger is-size-3 button" onClick={addHoleskyNetwork}>Please connect to Holesky</span>
                )}

            </div>
            <div className="container mt-5">
                {withdrawError && (
                    <div className="withdraw-error">{withdrawError}</div>
                )}
                {withdrawSuccess && (
                    <div className="withdraw-success">{withdrawSuccess}</div>
                )}{" "}
            </div>
            <div className="hero-body">
                <div className="container has-text-centered box">
                    <div className="field is-grouped mb-6">
                        <p className="control is-expanded">
                            <input className="input" readOnly type="text" value={walletAddress}
                                   placeholder="Enter your wallet address (0x...)" onChange={handleChangeWallet}/>
                        </p>
                        <p className="control">
                            <button
                                className="button is-info"
                                onClick={getGASPHandler}
                                disabled={!walletAddress || chainId !== "0x4268"}>
                                Get GASP Tokens
                            </button>
                        </p>
                    </div>
                    <div className="field is-grouped">
                        <p className="control is-expanded">
                            <input className="input" readOnly type="text" value={walletAddress}
                                   placeholder="Enter your wallet address (0x...)" onChange={handleChangeWallet}/>
                        </p>
                        <p className="control">
                            <button
                                className="button is-info"
                                onClick={getGETHHandler}
                                disabled={!walletAddress || chainId !== "0x4268"}>
                                Get GETH Tokens
                            </button>
                        </p>
                    </div>
                </div>
            </div>
            <div>
                <button className="button is-info mb-2 mr-2" onClick={() => addGaspToMetamsk()}
                        disabled={!walletAddress || chainId !== "0x4268"}>Add GASP to MetaMask
                </button>
                <button className="button is-info mb-2" onClick={() => addGethToMetamsk()}
                        disabled={!walletAddress || chainId !== "0x4268"}>Add GETH to MetaMask
                </button>
            </div>
        </section>

    );
}

export default App;